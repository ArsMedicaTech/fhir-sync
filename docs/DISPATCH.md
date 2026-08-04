# Dispatch webhook contract

The `dispatch` module sends reference-only, post-commit notifications to
configured HTTPS consumers after the FHIR sink successfully writes a resource
to HAPI. Consumers fetch authoritative data from the FHIR store themselves.

## Event body

```json
{
  "spec_version": 1,
  "resource_type": "Patient",
  "fhir_id": "123",
  "fhir_version_id": "4",
  "op": "upsert",
  "source": "oscar_binlog",
  "idempotency_key": "oscar:demographic:42:0000012345",
  "occurred_at": "2026-08-03T17:00:00Z",
  "fhir_base_url": "https://fhir.example.invalid/fhir"
}
```

- `spec_version` is `1`. Consumers must ignore unknown fields.
- `resource_type` is a string (`Patient` today, extensible to `Appointment` etc.).
- `fhir_id` is the HAPI-assigned resource `id`.
- `fhir_version_id` is the HAPI `meta.versionId`; omitted when unavailable.
- `op` is `upsert` or `delete`.
- `source` is `oscar_binlog`, `webhook`, or `grpc`.
- `idempotency_key` is opaque; consumers should treat it as an opaque string
  and use it for idempotency. It must not be parsed for meaning.
- `occurred_at` is an RFC3339 UTC timestamp.
- `fhir_base_url` is the base FHIR URL the consumer should query.

A `delete` notification means the resource was tombstoned via `active=false` on
the FHIR side (soft delete). `fhir_id` is present when the tombstone succeeded.

No names, phone numbers, addresses, or other PHI are present in the body or in
dispatch logs or DLQ records.

## Delivery semantics

- At-least-once delivery per enabled consumer.
- One tokio task per consumer; each consumer has its own mpsc channel.
- Ordering is best-effort per-consumer. Global ordering is NOT guaranteed.
- If a consumer's channel is full, the notification is dropped, a warning is
  logged, and a counter is incremented. The FHIR sink never blocks on dispatch.
- Retries use exponential backoff (`retry_base_ms * 2^attempt`, capped at
  `attempt.min(10)`), up to `retry_max_attempts`.
- 2xx = delivered. 4xx except 429 = permanent failure, written to that
  consumer's DLQ. 5xx, 429, and network errors = retryable.
- Permanent failures and exhausted retries are written to
  `{dead_letter_dir}/{consumer_name}.jsonl` with only `idempotency_key`,
  `fhir_id`, and `error`.

## Request headers

Each POST includes:

- `Content-Type: application/json`
- `X-FhirSync-Timestamp: <unix seconds>`
- `X-FhirSync-Signature: sha256=<hex hmac of "{timestamp}.{body}">`
- `X-FhirSync-Delivery: <uuid v4>`

`X-FhirSync-Delivery` is generated once per notification and is stable across
retries of that notification.

## Consumer implementation checklist

- Verify the HMAC signature using a constant-time comparison. The signed
  payload is `"{timestamp}.{body}"` where `{body}` is the exact request body.
- Reject timestamps outside a replay window of `<= 300` seconds.
- Store a record of processed `idempotency_key` values and ignore duplicates.
- Ignore unknown JSON fields (forward compatibility for `tenant` etc.).
- Do not treat `idempotency_key` as meaningful data; it is opaque.
- Expect reordering; always fetch current state from `fhir_base_url` using the
  provided `fhir_id` and `fhir_version_id` if needed.
