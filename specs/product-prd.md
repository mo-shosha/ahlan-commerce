# Product PRD - Create And List Products

Status: Draft
Version: 1
Owner: Catalog team
Scenarios: [product-scenarios.md](product-scenarios.md)

## Problem

A store owner needs a catalog of products before they can sell anything. Right now the only
way to know how products behave is to read the Rust code. This PRD writes down, in plain
words, what "creating a product" and "listing products" must do, so anyone (engineer,
product owner, QA) can check the behavior without reading code.

## Users

- **Store owner / admin**: adds products to the catalog and checks what is in it.
- **API client** (admin UI, scripts): calls the REST API on behalf of the store owner.

## User Stories

1. As a store owner, I want to create a product with a title, handle, price, and stock, so
   that I can start selling it.
2. As a store owner, I want to see all products I created, so that I can check my catalog is
   correct.
3. As a store owner, I want the system to stop me when I make a mistake (empty title, bad
   handle, negative price, reused handle), so that my catalog stays clean.

## Terms

| Term | Meaning |
|---|---|
| Handle | The short, URL-friendly name of a product, for example `coffee-mug`. Used in shop links. |
| Price in cents | The price as a whole number of cents. `1999` means 19.99. |
| Inventory quantity | How many items are in stock. |
| Published | `true` means the product is visible to shoppers. `false` means it is a draft. |
| Request ID | A unique ID returned with every error, so support can find the request in the logs. |

## Product Fields

When creating a product, the client sends:

| Field | Required | Rule |
|---|---|---|
| `title` | Yes | Must contain at least one character that is not a space. |
| `handle` | Yes | Only lowercase letters `a-z`, digits `0-9`, and single hyphens. Cannot start or end with a hyphen. Must be unique across all products. |
| `description` | No | Free text. Can be left out or sent as `null`. |
| `price_cents` | Yes | Whole number from `0` to `2147483647`. |
| `inventory_quantity` | Yes | Whole number from `0` to `2147483647`. |
| `published` | Yes | `true` or `false`. |

The system adds:

| Field | Value |
|---|---|
| `id` | A new unique ID (UUID). |
| `created_at`, `updated_at` | The time the product was created. |
| `published_at` | The creation time if `published` is `true`, otherwise `null`. |

## Acceptance Criteria

### AC-1 Valid create (PRD-PROD-001)

- Given valid product data, when the client creates the product, the API answers
  `201 Created`.
- The response contains the saved product with every field above.
- The product is saved, so it shows up in the product list afterwards.

### AC-2 Duplicate handle rejected (PRD-PROD-002)

- Given a product with handle `laptop` already exists, when the client creates another
  product with handle `laptop`, the API answers `409 Conflict`.
- The error code is `duplicate_product_handle`.
- The first product is not changed and no second product is saved.

### AC-3 List empty products (PRD-PROD-003)

- Given no products exist, when the client lists products, the API answers `200 OK` with
  an empty list: `{ "products": [] }`.
- An empty catalog is not an error.

### AC-4 List persisted products (PRD-PROD-004)

- Given products were created, when the client lists products, the API answers `200 OK`
  with every product, both published and drafts.
- Products are ordered oldest first (by creation time).
- Products are stored in the database, so they are still listed after the API restarts.

### AC-5 Invalid create input rejected (PRD-PROD-005)

- Given product data that breaks a rule in the Product Fields table (blank or missing
  title, bad handle, negative or too large number, broken JSON), when the client creates
  the product, the API answers `400 Bad Request`.
- The error code is `validation_failed` and the message tells the user what to fix.
- Nothing is saved.

## Error Response Shape

Every error uses the same shape:

```json
{
  "error": {
    "code": "validation_failed",
    "message": "Product title is required.",
    "request_id": "..."
  }
}
```

The response never contains internal details such as stack traces or file names.

## Out Of Scope

- Publishing / unpublishing an existing product and the published-only list.
- Updating or deleting products.
- Pagination, search, and filtering.
- Authentication and permissions.
- GraphQL (next chapter).

## Open Questions

1. Should `title` have a maximum length? Today there is no limit.
2. Should `price_cents` of `0` (free product) be allowed? Today it is allowed.
3. Should the product list be paginated once the catalog grows?
4. What should happen when two clients create the same handle at the exact same moment?
   The database unique rule should block the second one, but we have no test for it yet.
