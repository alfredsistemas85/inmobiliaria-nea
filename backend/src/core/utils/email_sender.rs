use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::env;

pub async fn send_email(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let smtp_user = env::var("SMTP_USER").unwrap_or_default();
    let smtp_pass = env::var("SMTP_PASS").unwrap_or_default();
    let smtp_port = env::var("SMTP_PORT").unwrap_or_else(|_| "587".to_string()).parse::<u16>().unwrap_or(587);
    let from_email = env::var("SMTP_FROM").unwrap_or_else(|_| smtp_user.clone());

    if smtp_user.is_empty() || smtp_pass.is_empty() {
        tracing::warn!("SMTP credentials missing, skipping real email send to {}", to);
        return Ok(());
    }

    let email = Message::builder()
        .from(from_email.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(String::from(body))?;

    let creds = Credentials::new(smtp_user, smtp_pass);

    // Open a remote connection to gmail
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?
            .port(smtp_port)
            .credentials(creds)
            .build();

    mailer.send(email).await?;
    tracing::info!("Email sent successfully to {}", to);
    Ok(())
}
