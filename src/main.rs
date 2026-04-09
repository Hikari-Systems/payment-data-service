mod config;
mod routes;

use actix_web::web;

pub struct AppState {
    pub pool: sqlx::PgPool,
}

async fn transfer_knex_state(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    const FINAL_KNEX_MIGRATION: &str = "202504181501_customerid";

    let knex_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1 FROM information_schema.tables
            WHERE table_schema = 'public' AND table_name = 'knex_migrations'
        )",
    )
    .fetch_one(pool)
    .await?;

    if !knex_exists {
        return Ok(());
    }

    let fully_migrated: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM knex_migrations WHERE name LIKE $1)",
    )
    .bind(format!("{}%", FINAL_KNEX_MIGRATION))
    .fetch_one(pool)
    .await?;

    if !fully_migrated {
        anyhow::bail!(
            "knex_migrations table exists but the final migration ({}) is not present. \
             The TypeScript service has not finished migrating this database. \
             Bring the TS service up to complete migrations before switching to Rust.",
            FINAL_KNEX_MIGRATION
        );
    }

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _sqlx_migrations (
            version        BIGINT PRIMARY KEY,
            description    TEXT NOT NULL,
            installed_on   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            success        BOOL NOT NULL,
            checksum       BYTEA NOT NULL,
            execution_time BIGINT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    let already_done: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM _sqlx_migrations WHERE version = 1)")
            .fetch_one(pool)
            .await?;

    if already_done {
        return Ok(());
    }

    let migrator = sqlx::migrate!("./migrations");
    let baseline = migrator
        .migrations
        .iter()
        .find(|m| m.version == 1)
        .expect("baseline migration (0001) not found in embedded migrator");

    sqlx::query(
        "INSERT INTO _sqlx_migrations
             (version, description, installed_on, success, checksum, execution_time)
         VALUES ($1, $2, NOW(), true, $3, 0)",
    )
    .bind(baseline.version)
    .bind(baseline.description.as_ref())
    .bind(baseline.checksum.as_ref())
    .execute(pool)
    .await?;

    tracing::info!(
        "Knex → sqlx migration state transferred: baseline (v{}) marked as applied",
        baseline.version
    );
    Ok(())
}

async fn index() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::load().expect("Failed to load config");

    hs_utils::healthcheck::check_subcommand(cfg.server.port);

    hs_utils::logging::init(&cfg.log.level);

    let pool = hs_utils::db::build_pool(&cfg.db).await?;
    transfer_knex_state(&pool).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = web::Data::new(AppState { pool });
    let port = cfg.server.port;

    hs_utils::server::run(port, move || {
        actix_web::App::new()
            .app_data(state.clone())
            .wrap(hs_utils::middleware::timing())
            .route("/healthcheck", web::get().to(|| async { "OK" }))
            .route("/", web::get().to(index))
            .configure(routes::configure)
    })
    .await
}
