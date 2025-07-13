use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::SqlitePool;
use crate::models::message::Message;
use serde_json::json;
use crate::utils::jwt::Claims;

pub async fn get_all_messages(
    State(pool): State<SqlitePool>,
    claims: Claims,
) -> impl IntoResponse {
    if claims.role != "Agent" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Only agents can view messages." })),
        ).into_response();
    }

    let result = sqlx::query_as::<_, Message>("SELECT * FROM messages ORDER BY timestamp DESC")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            eprintln!("Message fetch error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch messages" })),
            ).into_response()
        }
    }
}