use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct InternalNote {
    pub id: String,
    pub ticket_id: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateInternalNoteInput {
    pub ticket_id: String,
    pub content: String,
}