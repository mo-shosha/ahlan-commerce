## What I Expect to Learn from Ahlan Commerce 

- learn how to build a complete software project step by step using Rust and modern software engineering practices.

- Through Ahlan Commerce, I want to:

- Improve my Rust programming skills through practical implementation.
- Learn how to build REST APIs using Axum.
- Understand database design and PostgreSQL integration.
- Learn how to write clean, maintainable, and testable code.
- Understand error handling, logging, and observability.
- Learn how to document technical decisions and project requirements.
- Gain experience with frontend development and API integration.
- Understand background workers, caching, and deployment.
- Improve my debugging and code review skills.

## Chapter 11 notes (background worker)

What changed in my mental model:

- `POST /api/import-jobs` returning `202` only means "job accepted / queued", not "products imported".
- Durable job rows (`import_jobs`) are what make crash recovery possible. A spawned Tokio task
  inside the HTTP handler would disappear if the API process died.
- The worker owns file validation and product creation. The API only checks the request shape
  (`input_path` present and non-empty).
- Retries are explicit state: `failed -> queued` keeps `attempts`. After 3 attempts the worker
  stops claiming the job even if someone requeues it.
- Logs need `job_id` + `attempt` + `status` (and a safe reason on failure), not the whole file.