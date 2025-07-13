use imap::Client;
use native_tls::TlsConnector;
use std::collections::HashMap;
use mailparse::{parse_mail, MailHeaderMap};
use std::net::TcpStream;


pub fn fetch_unseen_emails() -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error + Send + Sync>> {

    let tls = TlsConnector::builder().build()?;

  
    let tcp = TcpStream::connect("imap.gmail.com:993")?;
    let tls_stream = tls.connect("imap.gmail.com", tcp)?;
    let client = Client::new(tls_stream);
    let mut session = client.login("your_email@gmail.com", "your_app_password").map_err(|e| e.0)?;

    session.select("INBOX")?;

    let messages = session.search("UNSEEN")?;

    let mut parsed_emails = Vec::new();

    for msg_id in messages.iter() {
        let message = session.fetch(msg_id.to_string(), "RFC822")?;
        let mail = message.iter().next().ok_or("No message found")?;
        let body = mail.body().ok_or("Message has no body")?;

        let parsed = parse_mail(body)?;

        let subject = parsed.headers.get_first_value("Subject").unwrap_or_default();
        let from = parsed.headers.get_first_value("From").unwrap_or_default();
        let body_text = parsed.get_body()?;

        parsed_emails.push(HashMap::from([
            ("from".into(), from),
            ("subject".into(), subject),
            ("body".into(), body_text),
        ]));
    }

    session.logout()?;
    Ok(parsed_emails)
}