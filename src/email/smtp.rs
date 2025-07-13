use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

pub async fn send_email(to: &str, subject: &str, body: &str) -> Result<(), String> {
    let email = Message::builder()
        .from("Support Team <support@example.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();

    let creds = Credentials::new("your_email@example.com".into(), "your_app_password".into());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(|e| format!("Send error: {:?}", e))?;
    Ok(())
}
