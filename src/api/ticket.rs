use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::SqlitePool;
use serde_json::json;
use uuid::Uuid;
use time::OffsetDateTime;
use crate::utils::jwt::Claims;
use crate::models::ticket::{Ticket, CreateTicketInput};
use axum::extract::Query;
use std::collections::HashMap;
use serde::Deserialize;

pub async fn create_ticket(
    State(pool): State<SqlitePool>,
    claims: Claims, 
    Json(payload): Json<CreateTicketInput>,
) -> impl IntoResponse {
    let customer_id = claims.sub;
    let now = OffsetDateTime::now_utc().to_string();

    let id = Uuid::new_v4().to_string();

    let result = sqlx::query!(
        "INSERT INTO tickets (id, title, description, status, priority, customer_id, created_at, updated_at)
         VALUES (?, ?, ?, 'Open', ?, ?, ?, ?)",
        id,
        payload.title,
        payload.description,
        payload.priority,
        customer_id,
        now,
        now
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "msg": "Ticket created", "ticket_id": id }))),
        Err(e) => {
            eprintln!("Create ticket error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to create ticket" })))
        }
    }
}

// pub async fn get_tickets(State(pool): State<SqlitePool>) -> Response {
//     let tickets = sqlx::query_as::<_, Ticket>("SELECT * FROM tickets")
//         .fetch_all(&pool)
//         .await;

//     match tickets {
//         Ok(data) => (StatusCode::OK, Json(data)).into_response(),
//         Err(e) => {
//             eprintln!("Fetch tickets error: {:?}", e);
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to fetch tickets" }))).into_response()
//         }
//     }
// }

pub async fn update_ticket(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTicketInput>, // reuse the same input for update
) -> impl IntoResponse {
    let now = OffsetDateTime::now_utc().to_string();

    let result = sqlx::query!(
        "UPDATE tickets SET title = ?, description = ?, priority = ?, updated_at = ? WHERE id = ?",
        payload.title,
        payload.description,
        payload.priority,
        now,
        id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({ "msg": "Ticket updated" }))),
        Err(e) => {
            eprintln!("Update ticket error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to update ticket" })))
        }
    }
}

// pub async fn delete_ticket(
//     Path(id): Path<String>,
//     State(pool): State<SqlitePool>,
// ) -> impl IntoResponse {
//     let result = sqlx::query!("DELETE FROM tickets WHERE id = ?", id)
//         .execute(&pool)
//         .await;

//     match result {
//         Ok(_) => (StatusCode::OK, Json(json!({ "msg": "Ticket deleted" }))),
//         Err(e) => {
//             eprintln!("Delete ticket error: {:?}", e);
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to delete ticket" })))
//         }
//     }
// }

pub async fn get_all_tickets(
    State(pool): State<SqlitePool>,
    claims: Claims,
) -> impl IntoResponse {
    if claims.role != "Agent" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Access denied: Agents only." })),
        )
        .into_response();
    }

    let result = sqlx::query_as::<_, Ticket>("SELECT * FROM tickets")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            eprintln!("Error fetching tickets: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch tickets" })),
            )
                .into_response()
        }
    }
}


pub async fn get_my_tickets(
    State(pool): State<SqlitePool>,
    claims: Claims,
) -> impl IntoResponse {
    if !claims.is_customer() {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Only customers can view their own tickets." })),
        )
        .into_response();
    }

    let result = sqlx::query_as!(
        Ticket,
        "SELECT * FROM tickets WHERE customer_id = ?",
        claims.sub
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch your tickets" })),
            )
            .into_response()
        }
    }
}

pub async fn search_tickets(
    State(pool): State<SqlitePool>,
    Query(params): Query<HashMap<String, String>>,
    claims: Claims,
) -> impl IntoResponse {
    if claims.role != "Agent" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Only agents can search tickets" })),
        ).into_response();
    }

    let mut query = String::from("SELECT * FROM tickets WHERE 1=1");
    let mut sqlx_query = sqlx::query_as::<_, Ticket>("");

    if let Some(priority) = params.get("priority") {
        query.push_str(" AND priority = ?");
        sqlx_query = sqlx::query_as::<_, Ticket>(&query);
        sqlx_query = sqlx_query.bind(priority);
    }

    if let Some(status) = params.get("status") {
        query.push_str(" AND status = ?");
        sqlx_query = sqlx::query_as::<_, Ticket>(&query);
        sqlx_query = sqlx_query.bind(status);
    }

    if let Some(keyword) = params.get("keyword") {
        query.push_str(" AND (title LIKE ? OR description LIKE ?)");
        sqlx_query = sqlx::query_as::<_, Ticket>(&query);
        sqlx_query = sqlx_query.bind(format!("%{}%", keyword));
        sqlx_query = sqlx_query.bind(format!("%{}%", keyword));
    }

    let result = sqlx_query.fetch_all(&pool).await;

    match result {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            eprintln!("Search error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to search tickets" })),
            ).into_response()
        }
    }
}

pub async fn ticket_stats(
    State(pool): State<SqlitePool>,
    claims: Claims,
) -> impl IntoResponse {
    if claims.role != "Agent" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Access denied: Agents only." })),
        ).into_response();
    }

    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM tickets")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let open = sqlx::query_scalar!("SELECT COUNT(*) FROM tickets WHERE status = 'Open'")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let closed = sqlx::query_scalar!("SELECT COUNT(*) FROM tickets WHERE status = 'Closed'")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    Json(json!({
        "total_tickets": total,
        "open_tickets": open,
        "closed_tickets": closed
    })).into_response()
}

#[derive(Deserialize)]
pub struct UpdateStatusPayload {
    pub status: String,
}

pub async fn update_ticket_status(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateStatusPayload>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "UPDATE tickets SET status = ? WHERE id = ?",
        payload.status,
        id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "msg": "Status updated" }))).into_response(),
        Err(e) => {
            eprintln!("Failed to update ticket status: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to update status" })),
            )
            .into_response()
        }
    }
}

pub async fn delete_ticket_by_id(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    claims: Claims,
) -> impl IntoResponse {
    if claims.role != "Agent" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Only agents can delete tickets." })),
        )
        .into_response();
    }

    let result = sqlx::query!("DELETE FROM tickets WHERE id = ?", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "msg": "Ticket deleted successfully." })),
        )
        .into_response(),
        Err(e) => {
            eprintln!("Delete ticket error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to delete ticket." })),
            )
            .into_response()
        }
    }
}

pub async fn create_ticket_from_email(
    pool: &SqlitePool,
    from: &str,
    subject: &str,
    body: &str,
) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = OffsetDateTime::now_utc().to_string();

    sqlx::query!(
        "INSERT INTO tickets (id, title, description, status, priority, customer_id, created_at, updated_at)
         VALUES (?, ?, ?, 'Open', 'Medium', ?, ?, ?)",
        id,
        subject,
        body,
        from,
        now,
        now
    )
    .execute(pool)
    .await?;

    Ok(())
}