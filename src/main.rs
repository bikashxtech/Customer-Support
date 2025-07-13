mod db;
mod api;
mod models;
mod services;
mod utils;
mod email;
mod background;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer};
use db::init_db;
use axum::routing::{post,put,delete};
use crate::api::auth::{register, login};
use dotenvy::dotenv;
use tokio::spawn;
use crate::background::email_fetcher::run_email_fetcher;
use crate::api::ticket::{create_ticket, update_ticket, delete_ticket_by_id, get_my_tickets, get_all_tickets, search_tickets, ticket_stats, update_ticket_status};
use crate::api::note::{add_internal_note, get_internal_notes};
use crate::api::public_note::{add_public_note, get_public_notes};
use crate::api::reply::agent_reply_to_ticket;
use serde::Deserialize;
use crate::api::channels::get_all_messages;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_pool = init_db().await;

    //spawn(run_email_fetcher(db_pool.clone()));
    let app = Router::new()
        .route("/", get(|| async { "Customer Support System Running 🚀" }))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/my-tickets", get(get_my_tickets))
        .route("/tickets", post(create_ticket).get(get_all_tickets))
        .route("/tickets/:id", post(update_ticket).delete(delete_ticket_by_id))
        .route("/internal-notes/add", post(add_internal_note))
        .route("/internal-notes/view", post(get_internal_notes))
        .route("/public-notes", post(add_public_note))
        .route("/public-notes/:ticket_id", get(get_public_notes))
        .route("/tickets/search", get(search_tickets))
        .route("/tickets/stats", get(ticket_stats))
        .route("/tickets/:id/status", put(update_ticket_status))
        .route("/messages", get(get_all_messages))
        .route("/tickets/:id/reply", post(agent_reply_to_ticket))
        .layer(CorsLayer::very_permissive())
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
    .await
    .unwrap();
}

