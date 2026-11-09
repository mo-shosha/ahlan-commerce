# Tracing And Observability

## 1. Logs vs. Spans vs. Events

- **Log:** A line of text written at one moment. Plain logs (`println!`, `log` + `env_logger`) have no idea which request they belong to.
- **Event:** A structured log line created with `tracing::info!`, `tracing::warn!`, or `tracing::error!`. It has a level, a message, and typed fields such as `product_id=...`.
- **Span:** A unit of work with a start and an end, such as one HTTP request. Every event emitted while the span is active is attached to it, so the event inherits the span's fields.

```text
http_request span (request_id, method, route, status, latency_ms, error_code)
     |
     +-- event: Product created (product_id, product_handle)
     |
     +-- event: HTTP request completed (status, latency_ms)
```

In our logs the span appears as the prefix `http_request{...}:` and the event comes after it.

---

## 2. Why `tracing` Fits Async Axum Services

- One Tokio worker thread runs many requests, switching between them at every `.await`.
- Thread-based logging cannot tell which request a line belongs to, because one thread serves many requests.
- A `tracing` span follows the request's future, not the thread. When the future resumes, its span is entered again, so events always land in the right request.
- `tower-http`'s `TraceLayer` creates the request span as Tower middleware, so handlers do not need any logging setup.

---

## 3. Where Things Live In Ahlan

| Concern | Location |
|---|---|
| Subscriber and `RUST_LOG` filter | `init_tracing()` in `apps/api/src/main.rs` |
| Request id generation | `request_id_middleware` in `apps/api/src/middleware.rs` |
| HTTP request span | `TraceLayer` in `apps/api/src/lib.rs` |
| `error_code` on the span | `AppError::into_response` in `apps/api/src/app_error.rs` |
| Product events | `create_product` in `apps/api/src/handlers.rs` |

The domain crate (`packages/catalog`) does not set up tracing. The API layer owns logging setup.

---

## 4. Fields That Help Debug Production

- **`request_id`:** Joins every log line for one request. It is also returned in the `x-request-id` header and in error bodies, so a user report can be matched to the logs.
- **`method` and `route`:** Show which endpoint is affected. `route` uses the matched pattern, not the raw URL.
- **`status` and `latency_ms`:** Show whether the request failed and whether it was slow.
- **`error_code`:** A stable category from the error contract (`validation_failed`, `duplicate_product_handle`, `not_found`, `internal_error`, ...). It is easy to search and count, unlike free-form messages.
- **`product_id` and `product_handle`:** Connect a request to the domain object it changed.
- **`operation`:** Names the action (`create_product`) so the same field works later for workers and cache operations.

Later chapters add `job_id` and `cache_key` only where that code owns them.

---

## 5. What We Do Not Log

- Full request bodies or raw customer payloads.
- Secrets, access tokens, or connection strings.
- Internal error causes in HTTP responses. The cause is logged on the server only, for `internal_error`.

---

## 6. Controlling Output With `RUST_LOG`

```powershell
$env:RUST_LOG="info"; cargo run -p api   # startup line, request spans, product events
$env:RUST_LOG="warn"; cargo run -p api   # only warnings and errors
$env:RUST_LOG="api=debug,tower_http=debug"; cargo run -p api   # more detail from our crate and tower-http
```

With `RUST_LOG=warn`, the `http_request` span is also hidden because it is created at `info` level. The `request_id` field is still on the error event, so failures stay traceable.

Captured examples: `docs/logs/success-request.log` and `docs/logs/failed-request.log`.
