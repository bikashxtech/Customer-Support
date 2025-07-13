🛠️ Customer Support System Backend (Rust + Axum)

A robust, scalable, and production-ready Customer Support Ticketing System backend written in Rust using Axum, SQLx, and SQLite. It supports multi-role user management, ticket creation, replies, internal/public notes, stats, and is built with extensibility in mind.

🚀 Features

✅ User Authentication (JWT)

🎫 Ticket Management (CRUD)

📬 Email Integration (IMAP fetcher + SMTP sending)

💬 Internal/Public Notes

📊 Ticket Stats & Search

📡 Multi-channel messaging (extensible)

🔔 Notification system (soon)

🧵 Background jobs for email polling

🧪 Modular Structure for scalability

📂 Project Structure

customer_support/
├── src/
│   ├── api/            # Route handlers (auth, tickets, notes, etc.)
│   ├── background/     # Email fetcher and background tasks
│   ├── email/          # IMAP and SMTP integration
│   ├── models/         # Data models
│   ├── services/       # Business logic
│   ├── utils/          # JWT, helpers
│   └── main.rs         # App entrypoint
├── .gitignore
├── Cargo.toml
├── README.md

🔧 Setup Instructions

✅ Prerequisites

Rust (latest stable) Install here

SQLite (already bundled via SQLx)

Git

📦 Install Dependencies

cargo build

⚙️ Environment Configuration

Create a .env file in the project root:

DATABASE_URL=sqlite://support.db
JWT_SECRET=your_super_secret_key
EMAIL_ADDRESS=your_email@gmail.com
EMAIL_PASSWORD=your_email_app_password

🚀 Run the App

cargo run

App runs at: http://localhost:3000

📬 Email Integration

Emails sent to the configured IMAP inbox will automatically generate tickets.

Fetcher runs in background using tokio::spawn

Incoming subject, from, body → Create ticket

Future: Reply from agent = outgoing SMTP email

📡 API Overview

Endpoint

Method

Auth

Description

/register

POST

No

Register new user

/login

POST

No

Login and receive JWT

/tickets

GET

Yes

Get all tickets (agent)

/my-tickets

GET

Yes

Get user's own tickets

/tickets/:id/reply

POST

Agent

Reply to ticket

/public-notes

POST

Agent

Add public note

/internal-notes/add

POST

Agent

Add internal note

Full API coming soon in docs/api.md

🧪 Development Tips

Enable SQLx offline mode: cargo sqlx prepare

Run only backend: cargo run

Use Postman or Thunder Client for testing

Use tokio-console for live async task viewing

🤝 Contributing

Pull requests welcome! Please open an issue first to discuss what you’d like to change.

🛡 License

MIT License

👨‍💻 Author

Bikash Kumar GiriBackend Developer | Rustacean | Data Science Enthusiast

GitHub | LinkedIn

