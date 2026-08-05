use std::time::{Duration, Instant};

use anyhow::{anyhow, bail, Context, Result};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::config::KeycloakConfig;

pub struct TokenProvider {
    client: reqwest::Client,
    token_url: String,
    client_id: String,
    client_secret: String,
    cached: Mutex<Option<CachedToken>>,
}

struct CachedToken {
    value: String,
    expires_at: Instant,
}

impl TokenProvider {
    pub fn new(cfg: &KeycloakConfig, client: reqwest::Client) -> Result<Self> {
        let client_secret = std::env::var(&cfg.client_secret_env)
            .with_context(|| format!("missing env var: {}", cfg.client_secret_env))?;
        if client_secret.is_empty() {
            bail!("env var {} is empty", cfg.client_secret_env);
        }

        Ok(Self {
            client,
            token_url: cfg.token_url.clone(),
            client_id: cfg.client_id.clone(),
            client_secret,
            cached: Mutex::new(None),
        })
    }

    pub async fn token(&self) -> Result<String> {
        let now = Instant::now();
        let cached = self.cached.lock().await;
        if let Some(c) = &*cached {
            if now < c.expires_at {
                return Ok(c.value.clone());
            }
        }
        drop(cached);

        let form = [
            ("grant_type", "client_credentials"),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
        ];

        let resp = self
            .client
            .post(&self.token_url)
            .form(&form)
            .send()
            .await
            .with_context(|| format!("POST {}", self.token_url))?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            bail!("token endpoint returned {status}: {text}");
        }

        let body: Value = resp
            .json()
            .await
            .with_context(|| "parsing token response as JSON")?;

        let access_token = body
            .get("access_token")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("token response missing access_token"))?;

        let expires_in = body
            .get("expires_in")
            .and_then(Value::as_u64)
            .ok_or_else(|| anyhow!("token response missing expires_in"))?;

        let ttl = Duration::from_secs(expires_in).saturating_sub(Duration::from_secs(30));
        let expires_at = Instant::now() + ttl;

        let mut cached = self.cached.lock().await;
        *cached = Some(CachedToken {
            value: access_token.to_string(),
            expires_at,
        });

        Ok(access_token.to_string())
    }
}

impl std::fmt::Debug for TokenProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenProvider").finish_non_exhaustive()
    }
}

#[cfg(test)]
impl TokenProvider {
    pub async fn cached_expires_in(&self) -> Option<Duration> {
        let cached = self.cached.lock().await;
        cached
            .as_ref()
            .map(|c| c.expires_at.saturating_duration_since(Instant::now()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::KeycloakConfig;
    use std::collections::VecDeque;
    use std::env;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::Mutex;
    use tokio::time::{sleep, Duration};

    fn response(status_line: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {}\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            body.len(),
            body
        )
    }

    fn queue(responses: Vec<String>) -> Arc<Mutex<VecDeque<String>>> {
        Arc::new(Mutex::new(responses.into_iter().collect()))
    }

    async fn spawn_server(
        responses: Arc<Mutex<VecDeque<String>>>,
        calls: Arc<AtomicUsize>,
    ) -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut buf = [0u8; 4096];
                let mut pos = 0;
                let mut header_end = None;
                while header_end.is_none() {
                    let n = socket.read(&mut buf[pos..]).await.unwrap();
                    pos += n;
                    if let Some(i) = buf[..pos]
                        .windows(4)
                        .position(|w| w == b"\r\n\r\n")
                    {
                        header_end = Some(i + 4);
                    }
                }
                let header_end = header_end.unwrap();

                let header = String::from_utf8_lossy(&buf[..header_end]);
                if let Some(len) = header
                    .to_lowercase()
                    .lines()
                    .find_map(|l| {
                        l.strip_prefix("content-length:")
                            .and_then(|v| v.trim().parse::<usize>().ok())
                    })
                {
                    let body_have = pos.saturating_sub(header_end);
                    let mut remaining = len.saturating_sub(body_have);
                    while remaining > 0 {
                        let to_read = remaining.min(4096);
                        let n = socket.read(&mut buf[..to_read]).await.unwrap();
                        remaining -= n;
                    }
                }

                calls.fetch_add(1, Ordering::Relaxed);
                let mut guard = responses.lock().await;
                let resp = guard.pop_front().unwrap_or_else(|| {
                    "HTTP/1.1 500 No more mocks\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        .to_string()
                });
                drop(guard);
                socket.write_all(resp.as_bytes()).await.unwrap();
            }
        });
        port
    }

    fn kc(port: u16, secret_env: &str) -> KeycloakConfig {
        KeycloakConfig {
            token_url: format!("http://127.0.0.1:{port}/token"),
            client_id: "fhir-sync".into(),
            client_secret_env: secret_env.into(),
        }
    }

    #[tokio::test]
    async fn cached_token_avoids_http_call() {
        let calls = Arc::new(AtomicUsize::new(0));
        let body = r#"{"access_token":"cached-token","expires_in":300}"#;
        let responses = queue(vec![response("200 OK", body)]);
        let port = spawn_server(responses, calls.clone()).await;

        env::set_var("FHIR_SYNC_TEST_SECRET_CACHE", "secret");
        let provider = TokenProvider::new(&kc(port, "FHIR_SYNC_TEST_SECRET_CACHE"), reqwest::Client::new()).unwrap();
        let t1 = provider.token().await.unwrap();
        let t2 = provider.token().await.unwrap();

        assert_eq!(t1, t2);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        env::remove_var("FHIR_SYNC_TEST_SECRET_CACHE");
    }

    #[tokio::test]
    async fn token_refetch_after_expiry() {
        let calls = Arc::new(AtomicUsize::new(0));
        let body = r#"{"access_token":"fresh-token","expires_in":1}"#;
        let responses = queue(vec![response("200 OK", body), response("200 OK", body)]);
        let port = spawn_server(responses, calls.clone()).await;

        env::set_var("FHIR_SYNC_TEST_SECRET_EXPIRE", "secret");
        let provider = TokenProvider::new(&kc(port, "FHIR_SYNC_TEST_SECRET_EXPIRE"), reqwest::Client::new()).unwrap();
        provider.token().await.unwrap();
        sleep(Duration::from_secs(2)).await;
        provider.token().await.unwrap();

        assert_eq!(calls.load(Ordering::Relaxed), 2);
        env::remove_var("FHIR_SYNC_TEST_SECRET_EXPIRE");
    }

    #[tokio::test]
    async fn thirty_second_buffer_applied_to_expires_in() {
        let calls = Arc::new(AtomicUsize::new(0));
        let body = r#"{"access_token":"buffer-token","expires_in":300}"#;
        let responses = queue(vec![response("200 OK", body)]);
        let port = spawn_server(responses, calls.clone()).await;

        env::set_var("FHIR_SYNC_TEST_SECRET_BUFFER", "secret");
        let provider = TokenProvider::new(&kc(port, "FHIR_SYNC_TEST_SECRET_BUFFER"), reqwest::Client::new()).unwrap();
        provider.token().await.unwrap();

        let remaining = provider.cached_expires_in().await.unwrap();
        assert!(
            remaining >= Duration::from_secs(269) && remaining <= Duration::from_secs(270),
            "expected ~270s remaining, got {remaining:?}"
        );
        env::remove_var("FHIR_SYNC_TEST_SECRET_BUFFER");
    }

    #[tokio::test]
    async fn missing_client_secret_env_fails_at_new() {
        env::remove_var("FHIR_SYNC_TEST_SECRET_MISSING");
        let cfg = KeycloakConfig {
            token_url: "http://127.0.0.1:1/token".into(),
            client_id: "fhir-sync".into(),
            client_secret_env: "FHIR_SYNC_TEST_SECRET_MISSING".into(),
        };
        let err = TokenProvider::new(&cfg, reqwest::Client::new()).unwrap_err();
        assert!(err.to_string().contains("FHIR_SYNC_TEST_SECRET_MISSING"));
    }

    #[tokio::test]
    async fn non_2xx_token_endpoint_returns_err() {
        let calls = Arc::new(AtomicUsize::new(0));
        let responses = queue(vec![response(
            "401 Unauthorized",
            r#"{"error":"invalid_client"}"#,
        )]);
        let port = spawn_server(responses, calls.clone()).await;

        env::set_var("FHIR_SYNC_TEST_SECRET_401", "secret");
        let provider = TokenProvider::new(&kc(port, "FHIR_SYNC_TEST_SECRET_401"), reqwest::Client::new()).unwrap();
        assert!(provider.token().await.is_err());
        env::remove_var("FHIR_SYNC_TEST_SECRET_401");
    }

    #[tokio::test]
    async fn malformed_response_returns_err() {
        let calls = Arc::new(AtomicUsize::new(0));
        let responses = queue(vec![response(
            "200 OK",
            r#"{"token_type":"Bearer"}"#,
        )]);
        let port = spawn_server(responses, calls.clone()).await;

        env::set_var("FHIR_SYNC_TEST_SECRET_MALFORMED", "secret");
        let provider = TokenProvider::new(&kc(port, "FHIR_SYNC_TEST_SECRET_MALFORMED"), reqwest::Client::new()).unwrap();
        assert!(provider.token().await.is_err());
        env::remove_var("FHIR_SYNC_TEST_SECRET_MALFORMED");
    }
}
