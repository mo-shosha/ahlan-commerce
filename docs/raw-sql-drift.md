# Raw SQL Drift

Task 07.1: add a published-products query and an update path that touches `description`
and `published_at`, still using plain SQL strings (`sqlx::query("...")`) in
`packages/catalog/src/store.rs`, on top of the Atlas-managed schema from Chapter 04.

## What Was Added

| Endpoint | What it does | SQL in `store.rs` |
|---|---|---|
| `GET /api/products/published` | only products with `published = true` | `list_published_products` |
| `PUT /api/products/{id}` | replaces title, handle, `description`, price, inventory, `published`; sets or clears `published_at` | `get_product` + `update_product` |
| `PATCH /api/products/{id}/published` | publishes a draft and stamps `published_at` (no-op if already published) | `get_product` + `update_product` |

`published_at` rules live in `Product::update` / `Product::publish` (`product.rs`):
draft to published sets it to now, published to draft clears it, and an edit of an
already-published product keeps the original timestamp.

```bash
curl -s localhost:3000/api/products/published
curl -s -X PUT localhost:3000/api/products/<id> -H 'content-type: application/json' \
  -d '{"title":"Phone","handle":"phone","description":"Unlocked.","price_cents":30000,"inventory_quantity":5,"published":true}'
curl -s -X PATCH localhost:3000/api/products/<id>/published
```

## Which SQL Strings Had to Change?

Three new strings, and none of the old ones could be reused:

- `list_published_products`: a copy of the list query plus `where published = true`.
- `get_product`: a copy of the list query plus `where id = $1`.
- `update_product`: `update ... set` with nine `$n` placeholders and a `returning` list.

The 10-column list `id, title, handle, description, price_cents, inventory_quantity,
published, published_at, created_at, updated_at` is now typed out **five times** (insert
`returning`, list, published list, get, update `returning`). The constraint name
`products_handle_key` is matched in two places. Each copy is plain text. If a column is renamed, all
five copies have to be found and fixed by hand, along with the `$n` numbering and the
`.bind(...)` order in insert and update.

## Which Rust Structs Had to Change?

For this task, no struct gained a field, because `description` and `published_at` were already
added in Chapter 04. That earlier change had to touch four structs plus one mapper, kept in
sync only by hand: `NewProduct`, `Product`, `CreateProductRequest`, `ProductResponse`, and
`product_from_row`.

This task changed:

- `Product`: new `update` and `publish` methods (the `published_at` rules).
- `CatalogError`: new `ProductNotFound` and `InvalidProductId` variants.
- `AppError::from_catalog`: maps them to `404 not_found` / `400 validation_failed`.
- `routes`: three new route constants. The `PUT` body reuses `CreateProductRequest`.

## What Could Compile While Still Being Wrong?

Everything below passes `cargo build`. Several were real bugs in the first draft of this task.

| Mistake | What happens |
|---|---|
| Route written as `/api/products/:id` (axum 0.7 syntax) | **Happened in this task.** Router panics at startup: `Path segments must not start with ':'`. Every endpoint is down, including `/health`. |
| Update path skipped the title/handle validation that create runs | **Happened in this task.** `PUT` stored a blank title or `Coffee Mug` as a handle. |
| `fetch_one` on `update ... where id = $1` | **Happened in this task.** A product deleted between read and write returned `500` instead of `404`. |
| Forgot `published_at = $8` in the `update` | `200 OK`, but an unpublished product keeps an old `published_at`. Silent bad data. |
| Swapped `.bind(&product.title)` / `.bind(&product.handle)` | Both are `text`. No error anywhere, title and handle silently swapped. Same for `price_cents` / `inventory_quantity` (both `integer`). |
| Typo `publised_at` in one query | Only that query fails, with `500` at runtime. |
| A column missing from one `select` list | `product_from_row` fails with `500`, only on that endpoint. |
| Nine `.bind` calls for ten placeholders | Runtime error from PostgreSQL. |
| Filter on `published = true` here but `published_at is not null` elsewhere | The schema has no check tying them together, so the two lists can disagree. |

The compiler sees each SQL string as a `&'static str`. It checks that `.bind()` gets a type
sqlx can encode, not that it matches the column, the placeholder number, or even the table.

## What Could Leak to Production If a Query Was Missed?

Suppose `db/schema/products.sql` changes, Atlas migrates cleanly, and one of the five queries is
not updated:

- **Drop or rename a column** (`description` to `body`): every query that names it returns
  `500`. `/health` still says `ok`, so a load balancer keeps routing traffic.
- **Add a `not null` column without a default**: every `create` fails. Reads and
  updates keep working, so this is a partial outage that is easy to miss.
- **Add a nullable column** (for example `compare_at_price_cents`): nothing errors. Inserts
  write `null`, updates never set it, and the data is silently missing.
- **Change what "visible" means** (for example add `deleted_at` for soft delete) and miss
  `list_published_products`: deleted or draft products keep showing up on the public
  storefront list. This is a real data leak, not a crash.
- **Change a type** (`price_cents` to `bigint`): `from_db_int` reads `i32`, so every read
  returns `500`.

The API tests catch these only for queries that a test actually runs. A missed query with no
test goes to production.

## Why Atlas Did Not Catch Missed Rust Query Updates

Atlas's inputs are `db/schema/products.sql`, `db/migrations/`, and the live database. It
never reads `.rs` files. It answers "does the database match the desired schema?" and never
"does the code match the database?"

- To **Atlas**, the queries in `store.rs` don't exist.
- To **rustc**, they are opaque strings. The workspace builds sqlx with default features off,
  so the compile-time `query!` / `query_as!` macros, which would check the SQL against a real
  database, are not in use.
- `atlas migrate lint` can warn that dropping a column is *destructive*. It cannot say that
  `store.rs` still selects that column in five places.

So the only link between the schema and the Rust code is the programmer's memory plus the
integration tests (`apps/api/tests/products_api.rs`, which run every query against the
Atlas-migrated schema). Atlas made schema changes safe and repeatable. Keeping queries in
sync with the schema needs a separate tool: compile-time checked queries, or a query layer
generated from the schema.
