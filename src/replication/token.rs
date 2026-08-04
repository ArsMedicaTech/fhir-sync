//! Cached bearer-token acquisition for replication nodes.
//! Supports static `token_env` and OAuth2 client-credentials with refresh.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::config::{NodeOAuthConfig, ReplicationNode};

pub struct TokenProvider {
    client: reqwest::Client,
    cache: Mutex<HashMap<String, CachedToken>>,
}

struct CachedToken {
    value: String,
    refresh_at: Instant,
    expires_at: Instant,
}

impl TokenProvider {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client,
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Returns a bearer token for `node`, refreshing if within the skew window.
    /// Precedence: `oauth` > `token_env` > `None`.
    pub async fn token_for(&self, node: &ReplicationNode) -> Option<String> {
        if let Some(oauth) = &node.oauth {
            return self.oauth_token(node, oauth).await.ok();
        }
        node.token_env
            .as_ref()
            .and_then(|key| std::env::var(key).ok())
    }

    async fn oauth_token(&self, node: &ReplicationNode, oauth: &NodeOAuthConfig) -> Result<String> {
        let now = Instant::now();

        let cached = self.cache.lock().await;
        if let Some(c) = cached.get(&node.name) {
            if now < c.refresh_at {
                return Ok(c.value.clone());
            }
        }
        drop(cached);

        let client_secret = std::env::var(&oauth.client_secret_env)
            .with_context(|| format!("missing env var: {}", oauth.client_secret_env))?;

        let mut form = vec![
            ("grant_type", "client_credentials"),
            ("client_id", oauth.client_id.as_str()),
            ("client_secret", client_secret.as_str()),
        ];
        if let Some(scope) = &oauth.scope {
            form.push(("scope", scope.as_str()));
        }

        let resp = self
            .client
            .post(&oauth.token_url)
            .form(&form)
            .send()
            .await
            .with_context(|| format!("POST {}", oauth.token_url))?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            let err = anyhow::anyhow!("token endpoint returned {status}: {text}");
            return self.fallback_or_error(&node.name, now, err).await;
        }

        let body: Value = resp
            .json()
            .await
            .with_context(|| "parsing token response as JSON")?;

        let access_token = body
            .get("access_token")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("token response missing access_token"))?;

        let expires_in = body
            .get("expires_in")
            .and_then(Value::as_u64)
            .ok_or_else(|| anyhow::anyhow!("token response missing expires_in"))?;

        let expires_at = now + Duration::from_secs(expires_in);
        let skew = (expires_in / 5).max(30);
        let refresh_at = now + Duration::from_secs(expires_in.saturating_sub(skew));

        let mut cached = self.cache.lock().await;
        cached.insert(
            node.name.clone(),
            CachedToken {
                value: access_token.to_string(),
                refresh_at,
                expires_at,
            },
        );

        Ok(access_token.to_string())
    }

    async fn fallback_or_error(
        &self,
        node_name: &str,
        now: Instant,
        err: anyhow::Error,
    ) -> Result<String> {
        let cached = self.cache.lock().await;
        if let Some(c) = cached.get(node_name) {
            if now < c.expires_at {
                tracing::warn!(
                    "replication node {}: token refresh failed, using cached token until expiry: {}",
                    node_name,
                    err
                );
                return Ok(c.value.clone());
            }
        }
        Err(err)
    }
}
