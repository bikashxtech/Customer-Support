use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    models::internal_note::{CreateInternalNoteInput, InternalNote},
    utils::jwt::Claims,
};

pub async fn add_internal_note(
    State(pool): State<SqlitePool>,
    claims: Claims,
    Json(payload): Json<CreateInternalNoteInput>,
) -> impl IntoResponse {
    if claims.role != "Agent" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Only agents can add internal notes." })),
        )
        .into_response();
    }

    let id = Uuid::new_v4().to_string();
    let now = OffsetDateTime::now_utc().to_string();

    let result = sqlx::query!(
        "INSERT INTO internal_notes (id, ticket_id, content, created_at) VALUES (?, ?, ?, ?)",
        id,
        payload.ticket_id,
        payload.content,
        now
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "msg": "Note added" }))).into_response(),
        Err(e) => {
            eprintln!("Add note error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to add note" })),
            )
            .into_response()
        }
    }
}

pub async fn get_internal_notes(
    State(pool): State<SqlitePool>,
    claims: Claims,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    if claims.role != "Agent" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Only agents can view internal notes." })),
        )
        .into_response();
    }

    let ticket_id = payload["ticket_id"].as_str().unwrap_or("");

    let result = sqlx::query_as!(
        InternalNote,
        "SELECT * FROM internal_notes WHERE ticket_id = ?",
        ticket_id
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(e) => {
            eprintln!("Fetch notes error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch notes" })),
            )
            .into_response()
        }
    }
}