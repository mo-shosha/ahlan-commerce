# ahlan-commerce

> A small learning project that teaches practical software engineering concepts through building an e-commerce application step by step.

## Layout

```text
apps/api/          HTTP transport: routes, config, DTOs, thin handlers
packages/catalog/  Product domain: product flows, UUIDv7 IDs, clock boundary
```

Request flow:

```text
route constant -> handler -> catalog domain function -> response DTO
```

## Run

```powershell
cargo run -p api
```

Configuration is read once at startup:

| Env key | Default | Meaning |
|---|---|---|
| `AHLAN_API_ADDR` | `0.0.0.0:3000` | Address the API binds to |
