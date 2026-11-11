# ahlan-commerce

> A small learning project that teaches practical software engineering concepts through building an e-commerce application step by step.

## Layout

```text
apps/api/          HTTP transport: routes, config, DTOs, thin handlers, DB pool setup
packages/catalog/  Product domain: product flows, UUIDv7 IDs, clock boundary, plain SQL store
db/schema/         Desired schema (source of truth for Atlas)
db/migrations/     Atlas-generated migrations
db/docker-init/    One-time init scripts for the PostgreSQL container
```

Request flow:

```text
route constant -> handler -> catalog domain function -> plain SQL (sqlx) -> PostgreSQL
                                                     -> response DTO
```

## Database

Products live in PostgreSQL 18, running in Docker (`docker-compose.yml`). The container
creates two databases on first start: `ahlan_commerce` for the app and
`ahlan_commerce_atlas_dev`, an empty scratch database Atlas uses to plan migrations.
Data survives restarts in the `db-data` volume.

```powershell
$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5433/ahlan_commerce"
docker compose up -d --wait            # make db-start
atlas migrate apply --env local        # make db-migrate
```

The container listens on host port `5433` because `5432` is used by the native
PostgreSQL service. Set `AHLAN_DB_PORT` before `docker compose up` to pick another port.

Open a `psql` shell inside the container with `docker compose exec db psql -U postgres -d ahlan_commerce`.
Run `docker compose down` to stop the container and keep its data. Run `docker compose down -v`
to delete the data too; the next `up` starts with empty databases again.

See `docs/manual-schema-change.md` for how schema changes flow through Atlas. The API
contract is in `docs/product-api-contract.md`.

## Run

Start PostgreSQL and the API together under [DeKit](https://dekit.run):

```powershell
make start   # opens the DeKit console with a PostgreSQL task and an Axum API task
make stop    # stops both; the database container and its data are kept
```

The API task starts only once PostgreSQL accepts connections. See `docs/commands.md`
for the individual targets.

To run the API on its own instead:

```powershell
$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5433/ahlan_commerce"
cargo run -p api
```

Configuration is read once at startup:

| Env key | Default | Meaning |
|---|---|---|
| `AHLAN_API_ADDR` | `0.0.0.0:3000` | Address the API binds to |
| `DATABASE_URL` | required | PostgreSQL connection URL |

The API connects at startup and fails fast if PostgreSQL is unreachable. One
connection pool (max 5 connections, 3 s acquire timeout) is shared through `AppState`.

## Test

API tests need `DATABASE_URL` too, and the database container must be running. Each
test creates its own throwaway schema, applies the files in `db/migrations/` to it, and
drops it afterwards, so tests never touch the `products` table the running API uses.

```powershell
cargo test --workspace
```
