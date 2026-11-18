# Ahlan Admin

React admin UI for the Ahlan product catalog.

## Stack

- **TanStack Router** — URL and screen state (`/products`)
- **TanStack Query** — server state, loading, errors, retries, invalidation
- **Thin GraphQL wrapper** — `POST /graphql` transport only

See [`docs/client-choice.md`](../../docs/client-choice.md) for why Apollo Client
is not the default.

## Run

API must already be listening on `http://127.0.0.1:3000` (for example
`make start` or `make run-api`).

```powershell
cd apps/admin
npm install
npm run dev
```

Open `http://localhost:5173/products`. Vite proxies `/graphql` to the API.

## E2E

```powershell
cd apps/admin
npx playwright install chromium
npm run test:e2e
```

Requires the GraphQL API on port 3000. The Playwright config builds the admin
app and serves it on port 4173.
