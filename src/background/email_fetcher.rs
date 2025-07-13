use crate::email::imap::fetch_unseen_emails;
use crate::api::ticket::create_ticket_from_email;
use sqlx::SqlitePool;
use tokio::time::{sleep, Duration};

pub async fn run_email_fetcher(pool: SqlitePool) {
    loop {
        match fetch_unseen_emails() {
            Ok(emails) => {
                for email in emails {
                    if let (Some(from), Some(subject), Some(body)) = (
                        email.get("from"),
                        email.get("subject"),
                        email.get("body"),
                    ) {
                        if let Err(e) = create_ticket_from_email(&pool, from, subject, body).await {
                            eprintln!("Error creating ticket from email: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch emails: {:?}", e);
            }
        }

        sleep(Duration::from_secs(60)).await;
    }
}