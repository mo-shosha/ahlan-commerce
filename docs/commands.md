
# Project Commands

This document describes the common development commands available in the project.

| Target | Description | Raw Command |
|---|---|---|
| `make run-api` | Runs the API server. | `cargo run -p api` |
| `make test` | Runs all workspace tests. | `cargo test --workspace` |
| `make lint` | Checks Rust formatting. | `cargo fmt --all -- --check` |
| `make db-start` | Starts PostgreSQL using Docker Compose. | `docker compose up -d` |
| `make db-stop` | Stops Docker Compose services. | `docker compose down` |
| `make db-migrate` | Applies pending Atlas migrations to the local database. | `atlas migrate apply --env local` |
| `make health` | Checks whether the API health endpoint responds successfully. | `curl --fail http://localhost:3000/health` |

## Notes

- PostgreSQL must be running before starting the API.
- `DATABASE_URL` must be configured before running the API.
- Atlas must be installed before running database migrations.
- The API must be running before executing the health check.
