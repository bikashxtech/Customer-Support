use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::SqlitePool;
use serde_json::json;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use time::OffsetDateTime;
use crate::models::user::{RegisterInput, LoginInput, User};
use crate::utils::jwt::generate_token;

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterInput>,
) -> impl IntoResponse {
    let hashed_pwd = hash(&payload.password, DEFAULT_COST).unwrap();
    let now = OffsetDateTime::now_utc().to_string();

    let id = Uuid::new_v4().to_string();
    let result = sqlx::query(
        "INSERT INTO users (id, name, email, password, role, created_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(id)
    .bind(payload.name)
    .bind(payload.email)
    .bind(hashed_pwd)
    .bind(payload.role)
    .bind(now)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "msg": "User registered!" }))),
        Err(e) => {
            eprintln!("Register error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Register failed" })))
        }
    }
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginInput>,
) -> impl IntoResponse {
    let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if let Some(user) = row {
        let is_valid = verify(&payload.password, &user.password).unwrap_or(false);
        if is_valid {
            let token = generate_token(&user.id, &user.role).unwrap();
            return (StatusCode::OK, Json(json!({ "token": token, "role": user.role, "id": user.id })));
        }
    }

    (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Invalid credentials" })))
}
