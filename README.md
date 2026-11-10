# ahlan-commerce

> A small learning project that teaches practical software engineering concepts through building an e-commerce application step by step.

## Layout

```text
apps/api/          HTTP transport: routes, config, DTOs, thin handlers, DB pool setup
packages/catalog/  Product domain: product flows, UUIDv7 IDs, clock boundary, plain SQL store
db/                Hand-written SQL schema files
```

Request flow:

```text
route constant -> handler -> catalog domain function -> plain SQL (sqlx) -> PostgreSQL
                                                     -> response DTO
```

## Database

Products live in a local PostgreSQL database named `ahlan_commerce`. The schema is
applied by hand from the files in `db/`, in this order (no migration tool yet):

```powershell
$psql = "C:\Program Files\PostgreSQL\18\bin\psql.exe"
& $psql -h localhost -U postgres -c "create database ahlan_commerce;"
& $psql -h localhost -U postgres -d ahlan_commerce -v ON_ERROR_STOP=1 -f db/initial-products.sql
& $psql -h localhost -U postgres -d ahlan_commerce -v ON_ERROR_STOP=1 -f db/add-description-and-published-at.sql
```

An existing database that already has `initial-products.sql` needs only the second file,
once. See `docs/manual-schema-change.md` for the order of steps and what goes wrong when
it is skipped. The API contract is in `docs/product-api-contract.md`.

## Run

```powershell
$env:DATABASE_URL = "postgres://postgres:<password>@localhost:5432/ahlan_commerce"
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

API tests need `DATABASE_URL` too. Each test creates its own throwaway schema from
`db/initial-products.sql` and drops it afterwards, so tests never touch the
`products` table the running API uses.

```powershell
cargo test --workspace
```
