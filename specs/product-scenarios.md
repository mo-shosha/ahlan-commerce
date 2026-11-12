# Product Scenarios - Create And List

Source PRD: [product-prd.md](product-prd.md)

## How To Read This File

- Each scenario has a stable ID like `PRD-PROD-001`. Never reuse or renumber an ID. If the
  behavior changes, bump the scenario `Version` instead.
- `Given` is the starting state, `When` is the one action, `Then` is what must be true after.
- `Verification` says how we know the scenario works:
  - `Automated by`: the tests that protect it. Each test is listed in exactly one scenario.
  - `Manual`: a person checks it by hand. The steps are written down.
  - `Future`: we want it checked but have not done it yet. The reason is written down.
  - `Blocked`: we cannot check it yet because something is missing. The blocker is written down.
- `Review` starts as `Pending`. A reviewer changes it to `Approved` or `Changes requested`.

Test locations:

| Short name | File | Kind |
|---|---|---|
| domain unit | `packages/catalog/src/service.rs` | Unit test, no database |
| record unit | `packages/catalog/src/records.rs` | Unit test, no database |
| id unit | `packages/catalog/src/id.rs` | Unit test, no database |
| DAL | `packages/db/tests/products_dal.rs` | Integration test, real PostgreSQL |
| API | `apps/api/tests/products_api.rs` | Integration test, HTTP + real PostgreSQL |

Run all of them with `make test` (PostgreSQL must be running: `make db-start`).

---

## PRD-PROD-001 - Valid create

**Version:** 1

**Intent:** A store owner can add a new product, and it is saved with all its data so it can
be sold later.

**Given**

- The catalog has no product with handle `laptop`.

**When**

- The client sends `POST /api/products` with:
  `title "Laptop"`, `handle "laptop"`, `description "Light laptop for travel."`,
  `price_cents 50000`, `inventory_quantity 10`, `published true`.

**Then**

- The API answers `201 Created`.
- The body is `{ "product": { ... } }` with the same title, handle, description, price,
  and quantity that were sent.
- The product has a new unique `id`.
- `created_at` and `updated_at` are the creation time.
- `published_at` is the creation time because `published` is `true`.
- If `published` is `false`, `published_at` is `null`.
- If `description` is left out or `null`, it is saved as `null`.

**Verification**

- Automated by:
  - API: `create_published_product_returns_contract_shape`
  - API: `unpublished_product_without_description_stores_nulls`
  - API: `null_description_is_stored_as_null`
  - domain unit: `new_product_copies_input_fields`
  - domain unit: `new_product_assigns_id_from_generator`
  - domain unit: `each_new_product_gets_the_next_id`
  - domain unit: `new_product_sets_both_timestamps_from_clock`
  - domain unit: `published_product_gets_published_at_from_clock`
  - domain unit: `unpublished_product_has_no_published_at`
  - domain unit: `new_product_accepts_url_safe_handles`
  - record unit: `record_round_trips_through_product`
  - id unit: `uuid_v7_generator_produces_version_7_ids`
  - id unit: `uuid_v7_generator_produces_unique_ids`
  - DAL: `create_returns_every_product_field`
- Manual: Start the stack with `make start`, send the request above with `curl`, and check
  the response matches the Then section.

**Review**

- Status: Pending
- Reviewer:
- Notes:

---

## PRD-PROD-002 - Duplicate handle rejected

**Version:** 1

**Intent:** Two products can never share a handle, because the handle is used in shop links
and must point to exactly one product.

**Given**

- A product with handle `laptop` already exists.

**When**

- The client sends `POST /api/products` with handle `laptop` again (any other fields).

**Then**

- The API answers `409 Conflict`.
- The error code is `duplicate_product_handle`.
- The message is `Another product already uses this handle.`
- The error has a non-empty `request_id`.
- The product list still has only the first `laptop` product, unchanged.

**Verification**

- Automated by:
  - API: `duplicate_handle_returns_conflict`
  - API: `duplicate_handle_keeps_only_the_first_product`
  - DAL: `create_rejects_a_duplicate_handle`
- Future: Two clients creating the same handle at the same moment. The database `unique`
  rule should stop the second one, but there is no concurrent test yet.

**Review**

- Status: Pending
- Reviewer:
- Notes:

---

## PRD-PROD-003 - List empty products

**Version:** 1

**Intent:** A brand new store with no products gets a normal, empty answer, not an error.

**Given**

- No products exist.

**When**

- The client sends `GET /api/products`.

**Then**

- The API answers `200 OK`.
- The body is exactly `{ "products": [] }`.

**Verification**

- Automated by:
  - API: `list_products_starts_empty`

**Review**

- Status: Pending
- Reviewer:
- Notes:

---

## PRD-PROD-004 - List persisted products

**Version:** 1

**Intent:** A store owner can see every product they created, in a stable order, and products
are not lost when the API restarts.

**Given**

- Product `laptop` (published) was created first.
- Product `phone` (draft, no description) was created second.

**When**

- The client sends `GET /api/products`.

**Then**

- The API answers `200 OK`.
- The list has 2 products: `laptop` first, then `phone`.
- Both published products and drafts are included.
- Every field is returned, including `description`, `published_at`, `created_at`, and
  `updated_at`.
- Products created at the same time are ordered by `id`.
- After the API process restarts, the same products are still listed.

**Verification**

- Automated by:
  - API: `list_returns_new_columns_in_creation_order`
  - API: `products_survive_an_api_restart`
  - DAL: `list_orders_by_created_at_then_id`
- Manual: Create a product, run `make db-stop` then `make db-start`, and list products again.
  The product must still be there (data lives in the `db-data` volume).

**Review**

- Status: Pending
- Reviewer:
- Notes:

---

## PRD-PROD-005 - Invalid create input rejected

**Version:** 1

**Intent:** Bad product data never enters the catalog, and the user is told what to fix.

**Given**

- Any catalog state.

**When**

- The client sends `POST /api/products` with one of these problems:
  - `title` is blank (only spaces) or missing.
  - `handle` has uppercase letters, spaces, `_`, `/`, accents, starts or ends with `-`,
    contains `--`, or is empty.
  - `price_cents` or `inventory_quantity` is negative.
  - `price_cents` or `inventory_quantity` is larger than `2147483647`.
  - The body is not valid JSON.

**Then**

- The API answers `400 Bad Request`.
- The error code is `validation_failed`.
- The message explains the problem, for example `Product title is required.` or
  `price_cents must be at most 2147483647.`
- The error has a non-empty `request_id`.
- No product is saved.

**Verification**

- Automated by:
  - API: `blank_title_returns_validation_error`
  - API: `missing_title_returns_validation_error`
  - API: `handle_that_is_not_lowercase_url_safe_returns_validation_error`
  - API: `negative_price_returns_validation_error`
  - API: `negative_inventory_returns_validation_error`
  - API: `price_above_integer_column_returns_validation_error`
  - API: `inventory_above_integer_column_returns_validation_error`
  - API: `malformed_json_returns_validation_error`
  - API: `invalid_create_does_not_save_a_product`
  - domain unit: `new_product_rejects_blank_title`
  - domain unit: `new_product_rejects_empty_title`
  - domain unit: `new_product_rejects_handles_that_are_not_lowercase_url_safe`
  - record unit: `price_above_integer_column_is_rejected`
- Blocked: Maximum title length. There is no rule for it yet (PRD open question 1), so there
  is nothing to test.

**Review**

- Status: Pending
- Reviewer:
- Notes:

---

## Tests Not Mapped To These Scenarios

These tests exist but protect behavior outside this PRD (see PRD "Out Of Scope"). They need
their own PRD and scenario IDs in a later chapter.

| Test | File | Why not mapped |
|---|---|---|
| `published_list_returns_only_published_products` | API | Publishing feature |
| `publishing_a_draft_sets_published_at_and_lists_it` | API | Publishing feature |
| `unpublishing_clears_published_at_and_hides_it` | API | Publishing feature |
| `publication_of_unknown_product_returns_not_found` | API | Publishing feature |
| `publication_with_malformed_id_returns_validation_error` | API | Publishing feature |
| `published_list_skips_drafts_and_puts_newest_publication_first` | DAL | Publishing feature |
| `update_publication_sets_only_publication_fields` | DAL | Publishing feature |
| `update_publication_of_unknown_product_returns_none` | DAL | Publishing feature |
| `negative_stored_quantity_is_rejected` | record unit | Reading corrupt data, not create/list input |
| `health_returns_ok` | API | Platform health check |
| `unknown_route_returns_not_found_error` | API | Platform error handling |
| `internal_failure_returns_safe_public_error` | API | Platform error handling |
| `uses_default_addr_when_unset` and the other 5 tests | `apps/api/src/config.rs` | Configuration, not product behavior |

## Coverage Summary

| Scenario | Automated | Manual | Future | Blocked |
|---|---|---|---|---|
| PRD-PROD-001 Valid create | Yes | Yes (curl check) | - | - |
| PRD-PROD-002 Duplicate handle | Yes | - | Concurrent create | - |
| PRD-PROD-003 List empty | Yes | - | - | - |
| PRD-PROD-004 List persisted | Yes | Yes (DB restart) | - | - |
| PRD-PROD-005 Invalid input | Yes | - | - | Max title length |
