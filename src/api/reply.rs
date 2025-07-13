use axum::{extract::{Path, State, Json}, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use sqlx::SqlitePool;
use crate::utils::jwt::Claims;
use crate::email::smtp::send_email;

#[derive(Deserialize)]
pub struct AgentReplyInput {
    message: String,
}

pub async fn agent_reply_to_ticket(
    Path(ticket_id): Path<String>,
    State(pool): State<SqlitePool>,
    claims: Claims,
    Json(input): Json<AgentReplyInput>,
) -> impl IntoResponse {
    // Agent check
    if claims.role != "Agent" {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": "Only agents can reply to tickets" })),
        )
        .into_response();
    }

    // Fetch customer email based on ticket_id
    let row = sqlx::query!(
        "SELECT customer_id FROM tickets WHERE id = ?",
        ticket_id
    )
    .fetch_optional(&pool)
    .await;

    let customer_email = match row {
        Ok(Some(record)) => record.customer_id,
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Ticket not found" })),
            )
            .into_response();
        }
    };

    // Send the email
    match send_email(
        &customer_email,
        &format!("Reply to your ticket [{}]", ticket_id),
        &input.message,
    )
    .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Reply sent to customer." })),
        ).into_response(),
        Err(e) => {
            eprintln!("Failed to send email: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to send reply email" })),
            )
            .into_response()
        }
    }
}