use crate::AppState;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct PaymentEvent {
    id: Uuid,
    provider_event_id: String,
    #[sqlx(json)]
    event_data: serde_json::Value,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

// eventData arrives as a JSON-encoded string — the TS client does JSON.stringify before sending.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateBody {
    provider_event_id: String,
    event_data: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateBody {
    provider_event_id: String,
    event_data: String,
}

async fn get_by_id(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let row = sqlx::query_as::<_, PaymentEvent>(
        "SELECT * FROM payment_event WHERE id = $1",
    )
    .bind(path.into_inner())
    .fetch_optional(&state.pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(match row {
        Some(e) => HttpResponse::Ok().json(e),
        None => HttpResponse::NoContent().finish(),
    })
}

async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateBody>,
) -> actix_web::Result<HttpResponse> {
    let event_data: serde_json::Value = serde_json::from_str(&body.event_data)
        .map_err(actix_web::error::ErrorBadRequest)?;

    let saved = sqlx::query_as::<_, PaymentEvent>(
        "INSERT INTO payment_event (id, provider_event_id, event_data, created_at)
         VALUES ($1, $2, $3, NOW()) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(&body.provider_event_id)
    .bind(sqlx::types::Json(&event_data))
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
    let event_data: serde_json::Value = serde_json::from_str(&body.event_data)
        .map_err(actix_web::error::ErrorBadRequest)?;

    let saved = sqlx::query_as::<_, PaymentEvent>(
        "UPDATE payment_event
         SET provider_event_id = $1, event_data = $2, updated_at = NOW()
         WHERE id = $3 RETURNING *",
    )
    .bind(&body.provider_event_id)
    .bind(sqlx::types::Json(&event_data))
    .bind(path.into_inner())
    .fetch_one(&state.pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(saved))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/paymentEvent/{id}", web::get().to(get_by_id))
        .route("/api/paymentEvent", web::post().to(create))
        .route("/api/paymentEvent/{id}", web::put().to(update));
}
