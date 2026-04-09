# payment-data-service-rs

A Rust/actix-web microservice for logging and retrieving Stripe (or other payment processor) webhook events and user payment state. Drop-in replacement for the original TypeScript/Express service; all API endpoints, JSON keys, and response shapes are identical.

---

## Features

- Payment event and user payment state CRUD backed by PostgreSQL
- JSONB column for `eventData` (full Stripe event payload)
- Automatic schema migrations on startup (safe against existing Knex-migrated databases via one-time state transfer)
- Config layering: baked defaults → `/sandbox/config.json` overlay → env vars
- Healthcheck subcommand built into the binary (no curl/wget needed in the container)

---

## API Endpoints

### `GET /healthcheck`
Returns `200 OK` with body `OK`.

```bash
curl http://localhost:3000/healthcheck
```

### `GET /`
Returns the service index page.

---

### Payment Event — `/api/paymentEvent`

#### `GET /api/paymentEvent/:id`
Returns a payment event by UUID. Returns `204 No Content` if not found.

```bash
curl http://localhost:3000/api/paymentEvent/550e8400-e29b-41d4-a716-446655440000
```

#### `POST /api/paymentEvent`
Creates a payment event. Returns `201 Created`.

```bash
curl -X POST http://localhost:3000/api/paymentEvent \
  -H 'Content-Type: application/json' \
  -d '{
    "providerEventId": "evt_1ABC123",
    "eventData": "{\"type\":\"checkout.session.completed\",\"data\":{}}"
  }'
```

`eventData` is a **JSON-encoded string** containing the full provider event payload.

| Field | Type | Required |
|---|---|---|
| `providerEventId` | string | yes |
| `eventData` | JSON-encoded string | yes |

**Response `201`**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "providerEventId": "evt_1ABC123",
  "eventData": { "type": "checkout.session.completed", "data": {} },
  "createdAt": "2025-04-10T00:00:00",
  "updatedAt": null
}
```

#### `PUT /api/paymentEvent/:id`
Updates a payment event. Returns `200 OK` with the updated object.

```bash
curl -X PUT http://localhost:3000/api/paymentEvent/550e8400-e29b-41d4-a716-446655440000 \
  -H 'Content-Type: application/json' \
  -d '{
    "providerEventId": "evt_1ABC123",
    "eventData": "{\"type\":\"checkout.session.completed\",\"data\":{\"updated\":true}}"
  }'
```

---

### User Payment State — `/api/userPaymentState`

#### `GET /api/userPaymentState/:id`
Returns a user payment state record by UUID. Returns `204 No Content` if not found.

```bash
curl http://localhost:3000/api/userPaymentState/550e8400-e29b-41d4-a716-446655440000
```

#### `GET /api/userPaymentState/byUserId/:userId`
Returns all payment state records for a user, ordered by `createdAt` descending.

```bash
curl http://localhost:3000/api/userPaymentState/byUserId/550e8400-e29b-41d4-a716-446655440000
```

#### `GET /api/userPaymentState/byUserIdAndSku/:userId/:sku`
Returns all payment state records for a user and SKU combination, ordered by `createdAt` descending.

```bash
curl http://localhost:3000/api/userPaymentState/byUserIdAndSku/550e8400-e29b-41d4-a716-446655440000/pro-monthly
```

#### `POST /api/userPaymentState`
Creates a user payment state record. Returns `201 Created`.

```bash
curl -X POST http://localhost:3000/api/userPaymentState \
  -H 'Content-Type: application/json' \
  -d '{
    "userId": "550e8400-e29b-41d4-a716-446655440000",
    "sku": "pro-monthly",
    "providerProductId": "prod_ABC123",
    "providerPriceId": "price_XYZ789",
    "customerId": "cus_DEF456",
    "plan": "pro",
    "paidAt": "2025-04-10T00:00:00.000Z",
    "expiresAt": "2025-05-10T00:00:00.000Z"
  }'
```

| Field | Type | Required |
|---|---|---|
| `userId` | UUID | yes |
| `sku` | string | yes |
| `providerProductId` | string | yes |
| `providerPriceId` | string | yes |
| `customerId` | string | yes |
| `plan` | string | yes |
| `paidAt` | ISO-8601 string | yes |
| `expiresAt` | ISO-8601 string | yes |
| `refundedAt` | ISO-8601 string | no |

**Response `201`**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "userId": "661f9511-f3ac-52e5-b827-557766551111",
  "sku": "pro-monthly",
  "providerProductId": "prod_ABC123",
  "providerPriceId": "price_XYZ789",
  "customerId": "cus_DEF456",
  "plan": "pro",
  "paidAt": "2025-04-10T00:00:00",
  "expiresAt": "2025-05-10T00:00:00",
  "refundedAt": null,
  "createdAt": "2025-04-10T00:00:00",
  "updatedAt": null
}
```

#### `PUT /api/userPaymentState/:id`
Updates a user payment state record. Returns `200 OK` with the updated object. Accepts the same fields as POST.

---

## Configuration

Config is loaded from `config.json` (working directory), then optionally deep-merged with `/sandbox/config.json`, then overridden by environment variables using `__` as the path separator with **exact camelCase key names**.

### `config.json` reference

```json
{
  "server": {
    "port": 3000
  },
  "log": {
    "level": "debug"
  },
  "db": {
    "database": "payment-data-service",
    "host": "payment-data-service-db",
    "port": "5432",
    "username": "payment-data-service",
    "password": "payment-data-service"
  }
}
```

### Environment variable examples

```bash
db__host=postgres
db__password=secret
db__ssl__enabled=true
db__ssl__caCertFile=/certs/ca.pem
log__level=info
server__port=8080
```

---

## Docker / Deployment

### Build and run locally

```bash
docker compose up --build
```

Service listens on `http://localhost:3001` (host) → `3000` (container).

### Multi-stage Dockerfile

1. **`deps`** (`rust:1-bookworm`) — stub `main.rs` compiled first to cache all dependencies as a Docker layer.
2. **`builder`** — real source compiled on top of the cached deps. Cold build: ~25–35 min; subsequent builds with dep-cache layer hit: ~2 min.
3. **`runtime`** (`debian:bookworm-slim`) — binary, migrations, and `config.json` only. Runs as unprivileged `appuser`. Filesystem is read-only; `/tmp` and `/run` are provided as `tmpfs`.

### Healthcheck

The binary includes a `healthcheck` subcommand that uses only stdlib TCP — no curl required in the runtime image:

```bash
/app/server healthcheck [hostname [port]]
```

---

## Development

### Prerequisites

- Rust 1.75+
- PostgreSQL 14+

### Run locally

```bash
# start postgres first, e.g. via docker compose up payment-data-service-db
db__host=localhost db__port=5432 cargo run
```

### Compile check (no database needed)

```bash
cargo check
```
