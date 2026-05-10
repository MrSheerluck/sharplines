use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::SinglePart,
    transport::smtp::authentication::Credentials,
};
pub async fn send_email(
    smtp_host: &str,
    smtp_port: u16,
    smtp_username: &str,
    smtp_password: &str,
    from: &str,
    to: &str,
    subject: &str,
    html: &str,
    _text: &str,
) -> Result<(), String> {
    let email = Message::builder()
        .from(
            from.parse()
                .map_err(|e| format!("Invalid from address: {}", e))?,
        )
        .to(to
            .parse()
            .map_err(|e| format!("Invalid to address: {}", e))?)
        .subject(subject.to_string())
        .singlepart(
            SinglePart::builder()
                .header(lettre::message::header::ContentType::TEXT_HTML)
                .body(html.to_string()),
        )
        .map_err(|e| format!("Failed to build email: {}", e))?;
    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(smtp_host)
        .map_err(|e| format!("Failed to create mailer: {}", e))?
        .port(smtp_port)
        .credentials(creds)
        .build();
    mailer
        .send(email)
        .await
        .map_err(|e| format!("Failed to send email: {}", e))?;
    Ok(())
}
pub fn verification_email(verification_url: &str) -> (String, String) {
    let html = format!(
        "<h1>Welcome to SharpLines</h1><p>Click <a href='{}'>here</a> to verify your email.</p><p>Expires in 24 hours.</p>",
        verification_url
    );
    let text = format!(
        "Welcome to SharpLines!\n\nClick this link to verify:\n{}\n\nExpires in 24 hours.",
        verification_url
    );
    (html, text)
}

pub fn password_reset_email(reset_url: &str) -> (String, String) {
    let html = format!(
        "<h1>Reset your SharpLines password</h1><p>Click <a href='{}'>here</a> to reset.</p><p>Expires in 1 hour.</p>",
        reset_url
    );
    let text = format!(
        "Reset your SharpLines password:\n\n{}\n\nExpires in 1 hour.",
        reset_url
    );
    (html, text)
}
