# Axum Runtime Model

## 1. Axum Runtime Architecture

Axum is built on top of three main components:

- **Tokio:** Handles asynchronous task execution and scheduling using a multi-threaded runtime.
- **Hyper:** Handles HTTP connections and request/response processing.
- **Tower:** Provides middleware and service abstractions such as tracing, timeouts, and authentication.

### Request Processing Flow

```text
Tokio Runtime
     |
     v
Hyper HTTP Server
     |
     v
Tower Middleware
     |
     v
Axum Router
     |
     v
Request Handler
     |
     v
Application Logic
```

### Where Do Handlers Run?

Axum handlers are asynchronous functions executed within Tokio's runtime.

- Tokio schedules asynchronous tasks across its worker threads.
- Each request does not get its own dedicated OS thread.
- When a handler reaches an `.await` point and an operation is pending, the task yields control, allowing other tasks to make progress.

---

## 2. Why Blocking Work Is Risky

Tokio uses cooperative scheduling, meaning asynchronous tasks should avoid blocking the runtime's worker threads.

### Blocking vs. Non-Blocking Operations

**Bad: Blocking the current OS thread**

```rust
std::thread::sleep(
    std::time::Duration::from_secs(5)
);
```

**Good: Asynchronous waiting**

```rust
tokio::time::sleep(
    std::time::Duration::from_secs(5)
).await;
```

Blocking operations prevent other tasks scheduled on the same worker thread from making progress.

This can lead to:

- Increased request latency.
- Reduced server throughput.
- Delayed responses.
- Runtime starvation if all worker threads become blocked.

### Handling Blocking Operations

For unavoidable blocking operations, use `tokio::task::spawn_blocking`:

```rust
tokio::task::spawn_blocking(|| {
    // Perform blocking operation
}).await;
```

This executes the blocking operation on a dedicated blocking thread pool instead of blocking an asynchronous worker thread.

---

## 3. Axum vs. Actix Web

Both frameworks support Tokio, but they differ in their HTTP handling and server architecture.

| Dimension | Axum | Actix Web |
|---|---|---|
| Async Runtime | Tokio | Tokio |
| HTTP Handling | Hyper | Actix HTTP |
| Middleware | Tower | Actix Middleware |
| Worker Model | Usually one shared multi-threaded Tokio runtime | Multiple workers, each with its own single-threaded runtime |
| State Sharing | Shared state using `Arc` and synchronization primitives | Per-worker or shared state using `Arc` |

### Axum

Axum typically runs inside a single multi-threaded Tokio runtime.

- Tokio schedules tasks across its worker threads.
- Tasks can make progress on different worker threads.
- Application state can be shared between handlers.

### Actix Web

Actix Web creates multiple server workers.

- Each worker runs its own single-threaded Tokio runtime.
- The application factory is instantiated separately for each worker.
- Blocking one worker affects requests handled by that worker, while other workers can continue processing requests.

---

## 4. Shared Application State

Our application uses shared state to manage in-memory product data.

```rust
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<Store>>,
}
```

### Understanding the Components

- **`Store`:** Maintains the in-memory product data.
- **`Mutex`:** Provides synchronized mutable access to the shared store.
- **`Arc`:** Allows multiple handlers to share ownership of the same store.

### Passing State to the Router

Application state is registered using `.with_state(state)`:

```rust
let app = Router::new()
    .route("/products", get(get_products))
    .with_state(state);
```

This allows handlers to access the shared application state through Axum's state extraction mechanism.

### Important Consideration

Avoid holding a synchronous `Mutex` lock across an `.await` point.

Holding a lock while waiting for asynchronous operations can block other tasks and potentially cause deadlocks.

Keep critical sections short and release locks before performing asynchronous work.

---

## 5. Engineering Takeaways

Understanding the runtime architecture helps build reliable and efficient asynchronous applications.

Key principles:

1. **Tokio** manages asynchronous task execution and scheduling.
2. **Hyper** handles HTTP communication.
3. **Tower** provides middleware and service abstractions.
4. **Axum** handles routing, request extraction, and handler execution.
5. Handlers should remain thin and delegate business logic to the domain layer.
6. Shared application state should be managed intentionally.
7. Blocking operations should not execute directly inside asynchronous handlers.
8. Avoid holding synchronous `Mutex` locks across `.await` points.
