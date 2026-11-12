# Data Access Layer (DAL)

How Ahlan talks to PostgreSQL: Atlas owns the tables, Cornucopia turns SQL queries into Rust,
and the `db` crate is the only code that runs them.

## Request Flow

```text
handler (apps/api)  ->  ProductCatalog (packages/catalog)  ->  db::products (packages/db)  ->  db-generated  ->  PostgreSQL
  HTTP + JSON            validation, ids, clock, rules          one function per query          Cornucopia output
```

- Handlers only call `ProductCatalog`. They contain no SQL and never import `db-generated`.
- `ProductCatalog` turns a `Product` into a `ProductRecord` and calls `db::products::*`.
- `db::products::*` wraps the generated query and turns its row into a `ProductRecord`.

## Where Things Live

| What | Where | Owned by |
| --- | --- | --- |
| Schema (desired tables) | `db/schema/products.sql` | Atlas |
| Migrations | `db/migrations/` (+ `atlas.sum`) | Atlas |
| SQL query files | `db/queries/products/*.sql` | Cornucopia (see `docs/query-contract.md`) |
| Generated Rust bindings | `packages/db/generated/` (crate `db-generated`) | Cornucopia. Never edit by hand |
| Cornucopia config | `cornucopia.toml` | Cornucopia |
| DAL | `packages/db/` (crate `db`) | Us |

## Atlas vs Cornucopia

| | Atlas | Cornucopia |
| --- | --- | --- |
| Question it answers | What should the **tables** look like? | What **queries** do we run, and what Rust types do they use? |
| Reads | `db/schema/products.sql` | `db/queries/**/*.sql` + the migrations |
| Writes | `db/migrations/*.sql` | `packages/db/generated/` |
| Changes the database? | Yes (`migrate apply`) | No. Uses a throwaway database and deletes it |
| Knows about Rust? | No | Yes, it generates Rust |

Atlas changes the database. Cornucopia changes Rust code. Neither does the other's job.

## When to Run Atlas

When you change a **table** in `db/schema/products.sql`:

```powershell
atlas migrate diff <name> --env local   # writes a new file in db/migrations/
make db-migrate                         # atlas migrate apply --env local
```

## When to Run Cornucopia

When you change a **query** in `db/queries/`, **or** after any new migration (a schema change
can break a query):

```powershell
make db-start              # Cornucopia needs a running PostgreSQL server
make cornucopia-generate
cargo build                # the DAL now sees the new generated types
```

`make cornucopia-generate` runs:

```text
cornucopia fresh --url postgres://postgres:postgres@localhost:5433 \
  --queries-path db/queries --destination packages/db/generated \
  --sync false --async true db/migrations/<every migration, in order>.sql
```

It creates a temporary database, applies every Atlas migration, checks each query against
it, writes `packages/db/generated/`, and drops the temporary database. Running it again with
no changes produces the same files. Commit the generated code together with the SQL change.

## How Changes Flow into Rust

**Schema change** (for example, rename `description` to `body`):

1. Edit `db/schema/products.sql`, then run `atlas migrate diff`.
2. `make cornucopia-generate` **fails**, because the queries still say `description`. Drift is
   caught here, not in production.
3. Fix the queries in `db/queries/products/`, then regenerate.
4. `cargo build` **fails** in `packages/db` where the generated field changed. Fix the DAL
   and `ProductRecord`.
5. Commit the migration, queries, generated code, and Rust changes together.

**Query change** (for example, a new ordering):

1. Edit the `.sql` file, then run `make cornucopia-generate`.
2. If the parameters or columns changed, `cargo build` points to every place to update.

## Why Handlers Do Not Own Persistence

- **One place for SQL.** Every query is in `db/queries/`, so a schema change has a short,
  known list of things to check.
- **Generated types stay hidden.** If Cornucopia's output or the driver changes, only
  `packages/db` changes. Handlers and domain code do not.
- **Handlers stay about HTTP.** They parse requests, call the catalog, and shape responses.
- **Rules live in one layer.** "`published_at` is set when published" is decided in the
  catalog, not repeated in each handler or query.
- **Easy to test.** The DAL is tested on its own (`packages/db/tests/`), and the API tests
  go through the real DAL.

## Rules

- Never edit `packages/db/generated/`. Change the SQL and regenerate.
- Mark nullable columns and parameters with `?` in the `--!` annotation. Without it the code
  compiles, then fails on the first `NULL`.
- No SQL strings outside `db/queries/`.
- Only `packages/db` depends on `db-generated`.
