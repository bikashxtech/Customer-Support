use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::SqlitePool;
use uuid::Uuid;
use time::OffsetDateTime;

use crate::models::public_note::{PublicNote, CreatePublicNoteInput};
use crate::utils::jwt::Claims;

pub async fn add_public_note(
    State(pool): State<SqlitePool>,
    claims: Claims,
    Json(payload): Json<CreatePublicNoteInput>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let now = OffsetDateTime::now_utc().to_string();

    let result = sqlx::query!(
        "INSERT INTO public_notes (id, ticket_id, author_id, content, created_at)
         VALUES (?, ?, ?, ?, ?)",
        id,
        payload.ticket_id,
        claims.sub,
        payload.content,
        now
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "msg": "Public note added", "id": id }))).into_response(),
        Err(e) => {
            eprintln!("Error adding public note: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to add public note" })),
            )
            .into_response()
        }
    }
}

pub async fn get_public_notes(
    Path(ticket_id): Path<String>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, PublicNote>(
        "SELECT * FROM public_notes WHERE ticket_id = ? ORDER BY created_at ASC"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(e) => {
            eprintln!("Error fetching public notes: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch notes" })),
            )
            .into_response()
        }
    }
}