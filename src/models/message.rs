use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: String,
    pub channel: String,      
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub timestamp: String,
    pub ticket_id: Option<String>,
}