# Manual Schema Change

## Atlas Solved Schema-Change Discipline, Not Rust Query Safety

The app still talks to PostgreSQL with plain SQL strings (`sqlx::query("...")` in
`packages/catalog/src/store.rs`). That is on purpose, for now.

**What Atlas fixed** (the pain from changing the schema by hand):

- One desired schema, `db/schema/products.sql`, instead of a pile of hand-written scripts.
- Migrations are generated, ordered, and stored in `db/migrations/`.
- The database records which versions ran (`atlas_schema_revisions`), so nobody has to remember.
- `atlas.sum` rejects edited migration files (`checksum mismatch`).
- Every environment gets the same steps: `atlas migrate apply --env local`.

**What Atlas did not fix:** Atlas knows the tables. It does not know the Rust code. The
SQL strings in `store.rs` are just text to both Atlas and the Rust compiler. If the schema
and the queries disagree, everything still builds, and the error shows up only when the
query runs:

| Change in `db/schema/products.sql` | `atlas migrate diff` / `apply` | `cargo build` | When the API runs |
|---|---|---|---|
| Drop the `description` column | ok | ok | `500`: `column "description" does not exist` |
| `price_cents` from `integer` to `bigint` | ok | ok | `500` on list: Rust reads `i32`, the column is now `int8` |
| Drop `not null` from `title` | ok | ok | `500` on list as soon as one row has a `null` title |

The first case is the same failure seen when changing the schema by hand:
`column "description" does not exist` on every product request, while `/health` said `ok`.

Today the column names and types still have to be kept in sync by hand, in the `insert`
column list, the `$1`..`$10` placeholders and the `bind` order, the `returning` and
`select` lists, `product_from_row`, and the constraint name `products_handle_key`. The only
safety net is the API test suite, which runs every query against the Atlas-migrated schema.
It catches drift only for the queries the tests actually exercise.

**The next pain is query/code drift.** The schema now changes safely, but every change
can still quietly break a query that nobody re-checked. The next step is to make the
queries themselves checked against the schema before the app runs, instead of finding
out in production.
