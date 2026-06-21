-- Consolidated baseline from 4 Knex migrations.
-- Safe on fresh DBs and existing Knex-migrated DBs (transfer_knex_state skips this on
-- Knex-managed databases, marking it applied without running it).
--
-- Column names are snake_case — what Knex produced via wrapIdentifier = snakeCase.
-- NOTE: this baseline declares plain TIMESTAMP, but the Rust models decode these columns as
-- DateTime<Utc> (i.e. they expect TIMESTAMPTZ). On existing Knex DBs the live columns are
-- already TIMESTAMPTZ (Knex t.timestamp() defaults to `with time zone`); on a FRESH DB built
-- from this file they are not. Migration 0002_timestamptz_guard.sql reconciles both cases.

CREATE TABLE IF NOT EXISTS payment_event (
    id                UUID         PRIMARY KEY NOT NULL,
    provider_event_id VARCHAR(100) NOT NULL,
    event_data        JSONB        NOT NULL,
    created_at        TIMESTAMP,
    updated_at        TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_payment_state (
    id                  UUID         PRIMARY KEY NOT NULL,
    user_id             UUID         NOT NULL,
    sku                 VARCHAR(100) NOT NULL,
    provider_product_id VARCHAR(100) NOT NULL,
    provider_price_id   VARCHAR(100) NOT NULL,
    paid_at             TIMESTAMP    NOT NULL,
    expires_at          TIMESTAMP    NOT NULL,
    created_at          TIMESTAMP,
    updated_at          TIMESTAMP,
    refunded_at         TIMESTAMP,
    customer_id         VARCHAR      NOT NULL DEFAULT '',
    plan                VARCHAR      NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS user_payment_state_user_id_sku
    ON user_payment_state (user_id, sku);
