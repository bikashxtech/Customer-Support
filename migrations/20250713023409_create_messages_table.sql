-- Add migration script here
CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    channel TEXT NOT NULL,           -- 'email', 'chat', 'twitter', etc.
    sender TEXT NOT NULL,
    recipient TEXT NOT NULL,
    content TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    ticket_id TEXT,                  -- Optional link to a support ticket
    FOREIGN KEY(ticket_id) REFERENCES tickets(id) ON DELETE SET NULL
);