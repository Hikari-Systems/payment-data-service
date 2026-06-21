-- Reconcile timestamp columns to TIMESTAMPTZ.
--
-- The 0001 baseline creates plain TIMESTAMP (WITHOUT TIME ZONE) on FRESH (non-Knex) DBs,
-- but the Rust models decode these columns as `DateTime<Utc>` — which sqlx only accepts for
-- TIMESTAMPTZ. So a fresh baseline DB fails to decode until this runs. On existing
-- Knex-migrated DBs the columns are ALREADY TIMESTAMPTZ (Knex `t.timestamp()` defaults to
-- `with time zone`), so this is a GUARDED NO-OP there: it only ALTERs columns still typed
-- `timestamp without time zone`, treating the stored naive value as UTC. It never shifts an
-- already-tz value.
DO $$
DECLARE r record;
BEGIN
  FOR r IN SELECT * FROM (VALUES
    ('payment_event','created_at'),
    ('payment_event','updated_at'),
    ('user_payment_state','paid_at'),
    ('user_payment_state','expires_at'),
    ('user_payment_state','created_at'),
    ('user_payment_state','updated_at'),
    ('user_payment_state','refunded_at')
  ) AS t(tbl, col) LOOP
    IF (SELECT data_type FROM information_schema.columns
        WHERE table_name = r.tbl AND column_name = r.col) = 'timestamp without time zone' THEN
      EXECUTE format(
        'ALTER TABLE %I ALTER COLUMN %I TYPE timestamptz USING %I AT TIME ZONE ''UTC''',
        r.tbl, r.col, r.col);
    END IF;
  END LOOP;
END $$;
