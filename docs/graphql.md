# GraphQL Slice

GraphQL is a typed query/mutation transport over the existing product catalog.
It does not replace the domain model.

```text
HTTP POST /graphql -> GraphQL schema -> resolver -> ProductCatalog -> DAL
```

Resolvers only shape requests and map errors. Product rules and SQL stay in
`packages/catalog` and `packages/db`.

## Endpoint

`POST /graphql`

## Example query

```graphql
query {
  products {
    id
    title
    handle
    description
    priceCents
    inventoryQuantity
    published
    publishedAt
    createdAt
    updatedAt
  }
}
```

```bash
curl -s http://localhost:3000/graphql \
  -H "content-type: application/json" \
  -d "{\"query\":\"{ products { id title handle description priceCents inventoryQuantity published publishedAt createdAt updatedAt } }\"}"
```

## Example mutation

```graphql
mutation {
  productCreate(
    input: {
      title: "Laptop"
      handle: "laptop"
      description: "Light laptop for travel."
      priceCents: 50000
      inventoryQuantity: 10
      published: true
    }
  ) {
    id
    title
    handle
    description
    priceCents
    inventoryQuantity
    published
    publishedAt
    createdAt
    updatedAt
  }
}
```

```bash
curl -s http://localhost:3000/graphql \
  -H "content-type: application/json" \
  -d "{\"query\":\"mutation { productCreate(input: { title: \\\"Laptop\\\", handle: \\\"laptop\\\", description: \\\"Light laptop for travel.\\\", priceCents: 50000, inventoryQuantity: 10, published: true }) { id title handle description priceCents inventoryQuantity published publishedAt createdAt updatedAt } }\"}"
```

## Errors

Domain failures use the Chapter 03A codes in GraphQL `extensions`:

| Code | When |
|---|---|
| `validation_failed` | Blank title, bad handle, negative/too-large numbers |
| `duplicate_product_handle` | Handle already exists |
| `not_found` | Missing resource (when a resolver needs one) |
| `dependency_unavailable` | Database unreachable |
| `internal_error` | Unexpected failure |

Example:

```json
{
  "data": null,
  "errors": [
    {
      "message": "Another product already uses this handle.",
      "extensions": {
        "code": "duplicate_product_handle",
        "request_id": "..."
      }
    }
  ]
}
```

## Schema

The expected SDL lives in [`apps/api/schema.graphql`](../apps/api/schema.graphql).
`DateTime` is an ISO-8601 UTC timestamp string such as `2026-01-01T00:00:00Z`.
