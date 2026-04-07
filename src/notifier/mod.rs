use crate::error::{NotifierError, Result};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
}

pub struct EmailNotifier {
    smtp_config: SmtpConfig,
}

impl EmailNotifier {
    pub fn new(smtp_config: SmtpConfig) -> Self {
        Self { smtp_config }
    }

    pub fn send(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        let from_addr: lettre::message::Mailbox = self
            .smtp_config
            .from
            .parse()
            .map_err(|e: lettre::address::AddressError| NotifierError::SendFailed(e.to_string()))?;
        let to_addr: lettre::message::Mailbox = to
            .parse()
            .map_err(|e: lettre::address::AddressError| NotifierError::SendFailed(e.to_string()))?;

        let email = Message::builder()
            .from(from_addr)
            .to(to_addr)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .map_err(|e: lettre::error::Error| NotifierError::SendFailed(e.to_string()))?;

        let creds = Credentials::new(
            self.smtp_config.username.clone(),
            self.smtp_config.password.clone(),
        );

        let mailer = SmtpTransport::relay(&self.smtp_config.host)
            .map_err(|e: lettre::transport::smtp::Error| NotifierError::SendFailed(e.to_string()))?
            .port(self.smtp_config.port)
            .credentials(creds)
            .build();

        mailer
            .send(&email)
            .map_err(|e: lettre::transport::smtp::Error| {
                NotifierError::SendFailed(e.to_string())
            })?;

        Ok(())
    }
}

pub struct NotificationQueue {
    notifier: Option<EmailNotifier>,
}

impl NotificationQueue {
    pub fn new(notifier: Option<EmailNotifier>) -> Self {
        Self { notifier }
    }

    pub fn notify(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        if let Some(ref notifier) = self.notifier {
            notifier.send(to, subject, body)
        } else {
            Err(NotifierError::SendFailed("No notifier configured".to_string()).into())
        }
    }

    pub fn is_configured(&self) -> bool {
        self.notifier.is_some()
    }
}
