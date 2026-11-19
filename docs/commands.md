
# Project Commands

This document describes the common development commands available in the project.

| Target | Description | Raw Command |
|---|---|---|
| `make start` | Starts PostgreSQL and the API under DeKit and opens its console. | `dekit up` then `dekit attach` |
| `make stop` | Stops every DeKit task and the DeKit runner (this stops the database container too). | `dekit down` |
| `make run-api` | Runs the API server. | `cargo run -p api` |
| `make run-worker` | Runs the product import worker. | `cargo run -p worker` |
| `make run-admin` | Runs the React admin UI (Vite). API must already be on port 3000. | `npm --prefix apps/admin run dev` |
| `make test` | Runs all workspace tests. | `cargo test --workspace` |
| `make test-admin-e2e` | Playwright e2e for admin create/list (API on port 3000). | `npm --prefix apps/admin run test:e2e` |
| `make lint` | Checks Rust formatting. | `cargo fmt --all -- --check` |
| `make db-start` | Starts PostgreSQL using Docker Compose and waits until it is healthy. | `docker compose up -d --wait` |
| `make db-run` | Starts PostgreSQL in the background, then follows its logs in the foreground. DeKit runs this as the `db` task. | `docker compose up --detach db` then `docker compose logs --follow db` |
| `make db-health` | Exits with 0 only when PostgreSQL accepts connections. | `docker compose exec -T db pg_isready --host=127.0.0.1 --username=postgres --dbname=ahlan_commerce` |
| `make db-stop` | Stops the PostgreSQL container. The container and the `db-data` volume are kept. | `docker compose stop db` |
| `make db-migrate` | Applies pending Atlas migrations to the local database. | `atlas migrate apply --env local` |
| `make cornucopia-generate` | Regenerates `packages/db/generated/` from `db/queries/`, checked against a temporary database built from `db/migrations/`. Safe to rerun. | `cornucopia fresh --url <server> --queries-path db/queries --destination packages/db/generated --sync false --async true db/migrations/*.sql` |
| `make health` | Checks whether the API health endpoint responds successfully. | `curl --fail http://localhost:3000/health` |

GraphQL examples (query and create mutation) are in `docs/graphql.md`.

## Notes

- Docker Desktop must be running before `make start`, `make db-start`, or `make db-run`.
- PostgreSQL must be running before starting the API. `make start` handles this: DeKit starts
  the API only after `make db-health` succeeds.
- The Makefile sets `DATABASE_URL` to `postgres://postgres:postgres@localhost:5433/ahlan_commerce`
  when it is not already set, and builds the port from `AHLAN_DB_PORT` (default `5433`).
  Set either variable to override it.
- Atlas must be installed before running database migrations.
- Cornucopia (`cargo install cornucopia`, 1.0+) must be installed, and PostgreSQL running,
  before `make cornucopia-generate`. Set `CORNUCOPIA_SERVER_URL` to use another server.
- The API must be running before executing the health check.
- The admin UI (`make run-admin` or `apps/admin` e2e) expects GraphQL at
  `http://127.0.0.1:3000/graphql`. Vite proxies `/graphql` in dev and preview.

## Running with DeKit

`dekit.yaml` at the repository root defines three tasks:

| Task | Command | Ready when |
|---|---|---|
| `db` (PostgreSQL) | `make db-run` | `make db-health` exits with 0 |
| `api` (Axum API) | `make run-api` | `GET http://localhost:3000/health` answers 2xx |
| `worker` (import worker) | `make run-worker` | process starts (`true` ready check) |

`api` and `worker` both depend on `db`, so DeKit waits for PostgreSQL to accept connections
before running them. Stopping the `db` task runs `make db-stop`, which stops the container.
The `docker compose logs --follow` process then exits by itself.

In the console, `q` asks how to quit: press `q` again to stop everything (like `make stop`)
or `d` to detach and leave the tasks running. Run `make start` again to reattach. Other
useful commands:


