# GraphQL Client Choice

Ahlan keeps frontend ownership explicit. The default admin app does **not** use
Apollo Client.

## Required path

| Piece | Owns |
|---|---|
| TanStack Router | URL and screen state |
| TanStack Query | Server state: cache keys, loading, errors, retries, invalidation |
| Thin GraphQL request wrapper | HTTP transport for GraphQL documents |

The wrapper only `POST`s a query/mutation and returns JSON. TanStack Query owns
when that call runs, how results are cached, and when the product list is
invalidated after create.

## Why not Apollo by default

Apollo Client is a rich GraphQL client with its own normalized cache and
tooling. Using it together with TanStack Query in a small exercise would create
two server-state cache owners and hide the split Ahlan is teaching.

Apollo Client is an **optional mentor extension** after the default task passes:
replace the thin request wrapper with Apollo Client while keeping the same
product list/create behavior. Do not mix both cache models unless the mentor is
explicitly teaching why both exist.
