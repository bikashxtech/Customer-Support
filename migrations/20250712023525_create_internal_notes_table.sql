-- Add migration script here
CREATE TABLE IF NOT EXISTS internal_notes (
    id TEXT PRIMARY KEY NOT NULL,
    ticket_id TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (ticket_id) REFERENCES tickets(id) ON DELETE CASCADE
);