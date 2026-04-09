# CLAUDE.md — payment-data-service-rs

Guidance for AI assistants working on this codebase.

---

## What this service does

Logs and retrieves Stripe (or other payment processor) webhook events and user payment state.
Exposes a REST API over PostgreSQL. No S3, no external HTTP calls, no file I/O beyond the database.
This is a Rust/actix-web rewrite of the original TypeScript/Express/Knex service and must remain a drop-in replacement (all API endpoints, JSON keys, and response shapes are identical).

---

## Codebase map

```
src/
  main.rs                         — startup: config load, pool build, Knex→sqlx state transfer, migrations, server bind
  config.rs                       — AppConfig loader: config.json → /sandbox/config.json overlay → env vars
  routes/
    mod.rs                        — registers all route groups
    payment_event.rs              — payment event CRUD; eventData as JSONB
    user_payment_state.rs         — user payment state CRUD; byUserId/byUserIdAndSku; NaiveDateTime timestamps
migrations/
  0001_baseline.sql               — both tables consolidated, IF NOT EXISTS throughout
config.json                       — default config baked into the image
static/
  index.html                      — embedded via include_str! at compile time
```

---

## Critical implementation decisions

### Config loading (`config.rs`)
**Do not use the `config` crate.** It normalises JSON keys to lowercase, which breaks `camelCase` fields. Config is loaded via `hs_utils::config` helpers:
1. `config.json` in the working directory (baked defaults)
2. `/sandbox/config.json` (deep-merged overlay, silently ignored if absent)
3. Env vars with `__` separator, exact camelCase path segments (e.g. `db__host`, `log__level`)

### Shared utilities (`hs_utils` v0.2.2)
All shared infrastructure comes from `hs-utils-rs` via `git+tag` reference — never a path dep. Modules used:
- `hs_utils::config` — config loading helpers (`prepare_config`, `apply_env_overrides`, `deep_merge`)
- `hs_utils::db` — `DbConfig` struct, `build_pool()`
- `hs_utils::healthcheck` — `check_subcommand(port)` (stdlib TCP, no curl in container)
- `hs_utils::logging` — `init(level)`
- `hs_utils::middleware` — `timing()` middleware
- `hs_utils::server` — `run(port, factory)`

### Knex → sqlx state transfer (`main.rs: transfer_knex_state`)
On first boot against a Knex-migrated database, `transfer_knex_state(&pool)` runs before `sqlx::migrate!().run()`:

1. If `knex_migrations` table does not exist → fresh DB, do nothing (sqlx runs baseline normally).
2. If `knex_migrations` exists but the final TS migration (`202504181501_customerid`) is absent → bail with error.
3. Otherwise: `CREATE TABLE IF NOT EXISTS _sqlx_migrations (...)`, then insert a synthetic row for version 1 using the **checksum extracted from the embedded migrator**, so future checksum validation passes.

Idempotent: safe on every boot; re-runs are no-ops.

### Route registration (`routes/mod.rs`)
Routes are registered directly on `web::ServiceConfig`, **never inside `web::scope("")`** (empty scope swallows 404s).

Route order is critical in `user_payment_state.rs` — specific paths must come before the wildcard `{id}`:
1. `/byUserIdAndSku/{userId}/{sku}` — FIRST
2. `/byUserId/{userId}` — SECOND
3. `/{id}` — LAST

### sqlx queries
Use **runtime queries** (`query_as::<_, Row>(sql).bind(value)`) not compile-time macros (`query_as!`). The macro requires `DATABASE_URL` at compile time, which breaks Docker builds without a live database.

### JSONB column (`event_data`)
- `event_data: serde_json::Value` annotated `#[sqlx(json)]` on the `PaymentEvent` FromRow struct
- The client sends `eventData` as a JSON-encoded **string** — `serde_json::from_str` in the handler before inserting
- Bind for INSERT/UPDATE: `sqlx::types::Json(&parsed_value)`

### Timestamps
All timestamp columns are `TIMESTAMP WITHOUT TIME ZONE` — Knex `t.timestamps()` and `t.timestamp()` produce timezone-less columns. Use `chrono::NaiveDateTime` (not `DateTime<Utc>`). Client sends ISO-8601 strings with timezone offset (e.g. `"2025-01-01T00:00:00.000Z"`); parse with `DateTime::parse_from_rfc3339` then `.naive_utc()`.

### Static files
`static/index.html` is embedded in the binary via `include_str!("../static/index.html")` at compile time. It does NOT need to be copied into the runtime Docker image. The `COPY static ./static` step in the Dockerfile must occur in the builder stage before `cargo build`.

---

## Response codes

| Endpoint | Code |
|---|---|
| GET (found) | 200 |
| POST | 201 |
| PUT | 200 |
| GET (not found) | 204 |

---

## Database schema (PostgreSQL snake_case names)

Knex's `wrapIdentifier` converted camelCase identifiers to snake_case in the original service.

| Logical name | Postgres table |
|---|---|
| paymentEvent | `payment_event` |
| userPaymentState | `user_payment_state` |

---

## Docker build notes

Two-stage build (no native deps beyond `ca-certificates`):
1. **`deps`** (`rust:1-bookworm`) — stub `src/main.rs` (`fn main(){}`) compiled to cache all dependencies as a Docker layer. Binary name to remove is `server` (the `[[bin]] name` from Cargo.toml).
2. **`builder`** — real source + `static/` + `migrations/` compiled on top of the cached dep layer.
3. **`runtime`** (`debian:bookworm-slim`) — binary, migrations, `config.json` only. Runs as unprivileged `appuser`. Container runs `read_only: true`; requires `tmpfs: [/tmp, /run]`.

Cold build: ~25–35 min. Subsequent builds with dep-cache layer hit: ~2 min.

---

## Adding new endpoints

1. Add the handler to the relevant `routes/` file.
2. Register the route in that file's `configure()`, placing static paths before wildcard `{id}` paths.
3. If a new migration is needed, add `000N_description.sql` to `migrations/`; never edit `0001_baseline.sql` after deployment.
