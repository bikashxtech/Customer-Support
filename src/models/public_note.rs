use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicNote {
    pub id: String,
    pub ticket_id: String,
    pub author_id: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePublicNoteInput {
    pub ticket_id: String,
    pub content: String,
}