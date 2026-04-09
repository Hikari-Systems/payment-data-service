use crate::AppState;
use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct UserPaymentState {
    id: Uuid,
    user_id: Uuid,
    sku: String,
    provider_product_id: String,
    provider_price_id: String,
    paid_at: NaiveDateTime,
    expires_at: NaiveDateTime,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    refunded_at: Option<NaiveDateTime>,
    customer_id: String,
    plan: String,
}

// Timestamps arrive as ISO-8601 strings (e.g. "2025-01-01T00:00:00.000Z").
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateBody {
    user_id: Uuid,
    sku: String,
    provider_product_id: String,
    provider_price_id: String,
    customer_id: String,
    plan: String,
    paid_at: String,
    expires_at: String,
    refunded_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateBody {
    user_id: Uuid,
    sku: String,
    provider_product_id: String,
    provider_price_id: String,
    customer_id: String,
    plan: String,
    paid_at: String,
    expires_at: String,
    refunded_at: Option<String>,
}

fn parse_dt(s: &str) -> actix_web::Result<NaiveDateTime> {
    // Try RFC3339 first (handles "2025-01-01T00:00:00.000Z" and with tz offsets).
    // .naive_utc() drops the timezone info — correct for TIMESTAMP WITHOUT TIME ZONE columns.
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.naive_utc())
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
        .map_err(actix_web::error::ErrorBadRequest)
}

fn parse_opt_dt(s: Option<&str>) -> actix_web::Result<Option<NaiveDateTime>> {
    match s.filter(|s| !s.trim().is_empty()) {
        Some(s) => parse_dt(s).map(Some),
        None => Ok(None),
    }
}

async fn get_by_id(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let row = sqlx::query_as::<_, UserPaymentState>(
        "SELECT * FROM user_payment_state WHERE id = $1",
    )
    .bind(path.into_inner())
    .fetch_optional(&state.pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(match row {
        Some(r) => HttpResponse::Ok().json(r),
        None => HttpResponse::NoContent().finish(),
    })
}

async fn get_by_user_id(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let rows = sqlx::query_as::<_, UserPaymentState>(
        "SELECT * FROM user_payment_state WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(path.into_inner())
    .fetch_all(&state.pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(rows))
}

async fn get_by_user_id_and_sku(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, String)>,
) -> actix_web::Result<HttpResponse> {
    let (user_id, sku) = path.into_inner();
    let rows = sqlx::query_as::<_, UserPaymentState>(
        "SELECT * FROM user_payment_state WHERE user_id = $1 AND sku = $2 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .bind(&sku)
    .fetch_all(&state.pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(rows))
}

async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateBody>,
) -> actix_web::Result<HttpResponse> {
    let paid_at = parse_dt(&body.paid_at)?;
    let expires_at = parse_dt(&body.expires_at)?;
    let refunded_at = parse_opt_dt(body.refunded_at.as_deref())?;

    let saved = sqlx::query_as::<_, UserPaymentState>(
        "INSERT INTO user_payment_state
         (id, user_id, sku, provider_product_id, provider_price_id,
          customer_id, plan, paid_at, expires_at, refunded_at, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW()) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(body.user_id)
    .bind(&body.sku)
    .bind(&body.provider_product_id)
    .bind(&body.provider_price_id)
    .bind(&body.customer_id)
    .bind(&body.plan)
    .bind(paid_at)
    .bind(expires_at)
    .bind(refunded_at)
    .fetch_one(&state.pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(saved))
}

async fn update(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateBody>,
) -> actix_web::Result<HttpResponse> {
    let paid_at = parse_dt(&body.paid_at)?;
    let expires_at = parse_dt(&body.expires_at)?;
    let refunded_at = parse_opt_dt(body.refunded_at.as_deref())?;

    let saved = sqlx::query_as::<_, UserPaymentState>(
        "UPDATE user_payment_state
         SET user_id = $1, sku = $2, provider_product_id = $3,
             provider_price_id = $4, customer_id = $5, plan = $6,
             paid_at = $7, expires_at = $8, refunded_at = $9,
             updated_at = NOW()
         WHERE id = $10 RETURNING *",
    )
    .bind(body.user_id)
    .bind(&body.sku)
    .bind(&body.provider_product_id)
    .bind(&body.provider_price_id)
    .bind(&body.customer_id)
    .bind(&body.plan)
    .bind(paid_at)
    .bind(expires_at)
    .bind(refunded_at)
    .bind(path.into_inner())
    .fetch_one(&state.pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(saved))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    // CRITICAL: specific paths before the /{id} wildcard — actix-web matches in order.
    cfg.route(
            "/api/userPaymentState/byUserIdAndSku/{userId}/{sku}",
            web::get().to(get_by_user_id_and_sku),
        )
        .route(
            "/api/userPaymentState/byUserId/{userId}",
            web::get().to(get_by_user_id),
        )
        .route("/api/userPaymentState/{id}", web::get().to(get_by_id))
        .route("/api/userPaymentState", web::post().to(create))
        .route("/api/userPaymentState/{id}", web::put().to(update));
}
