# Data Access Layer Alternatives

Task 07.1 showed the problem with plain SQL strings: the compiler and Atlas never check
them, so a wrong column, a swapped bind, or a missed query only fails when the app runs.

This page compares ways to fix that.

| Option | Pros | Cons | Protects against | Does not protect against | Fit for Ahlan |
| --- | --- | --- | --- | --- | --- |
| Diesel | Strong compile-time checks. Mature. | Sync by default. Hard compile errors. Complex SQL is awkward. Has its own migrations. | Wrong column names and types. | A stale `schema.rs`. Wrong filters. Business rules. | Valid, not default |
| SeaORM | Async. Easy CRUD. | Mostly checked at runtime. Hides the SQL. Has its own migrations. | Column typos in Rust code. | Entities that drift from the database. Wrong filters. Business rules. | Not a fit |
| SQLx checked queries | Same driver and SQL as today. Checked at `cargo build`. | SQL inside Rust strings. Positional `$1, $2` binds. Needs a database or `.sqlx/` cache to build. | Wrong columns, wrong types, wrong bind count. | Swapped binds of the same type. Wrong filters. Business rules. | Runner-up |
| SQL-first code generation pattern | SQL in `.sql` files. Typed Rust is generated from it. Full PostgreSQL. | Generated code must be regenerated and committed. | Wrong columns, wrong types, wrong parameters. | Stale generated code. Wrong filters. Business rules. | The approach we follow |
| Cornucopia | SQL-first for PostgreSQL. Named parameters (`:title`). Checked against a real database. | Uses `tokio-postgres` instead of SQLx. Columns are non-null unless marked with `?`. | Wrong columns, wrong types, swapped parameters. | A missing `?` on a nullable column. Stale generated code. Wrong filters. Business rules. | Ahlan default |

## Why an ORM Is Valid but Not the Default

ORMs like Diesel and SeaORM work well, and many Rust projects use them. Ahlan does not use
one by default because:

- **Atlas already owns the schema.** An ORM keeps a second copy of the tables in Rust, which
  must be kept in sync after every migration.
- **We want real SQL.** Commerce needs PostgreSQL features (`on conflict`, `for update`)
  that ORMs make awkward.
- **We want to see the SQL** that runs, so we can review and tune it.

## Why Ahlan Uses Cornucopia

- Atlas keeps the schema in SQL, and Cornucopia keeps the queries in SQL.
- Each query is written once, and Cornucopia generates the Rust function and row type.
- Named parameters stop the swapped-bind bug that SQLx macros still allow.
- `cargo build` does not need a database.

## Why SQL-First Still Needs Discipline

Generated code is only correct if you keep it up to date.

- **Regenerate after every schema change**, and let CI fail if the generated code is stale.
- **Mark nullable columns with `?`** (`description?`, `published_at?`). If you forget, the
  code compiles, then fails on the first `NULL`.
- **Never edit generated code.** Change the `.sql` file and regenerate.
- **Put all SQL in the query files.** One raw query string brings back the Task 07.1 problems.
- **Keep writing tests.** Generation proves a query runs, not that it returns the right rows.
