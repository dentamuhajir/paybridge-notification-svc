use crate::domain::notification::port::EmailSender;
use anyhow::Result;
use lettre::{
    message::{header::ContentType, SinglePart},
    Message, SmtpTransport, Transport,
};

pub struct MailhogEmailSender {
    mailer: SmtpTransport,
    from: String,
}

impl MailhogEmailSender {
    /// Creates a new MailhogEmailSender instance.
    ///
    /// Accepts `smtp_host`, `smtp_port`, and `smtp_from` from Config
    /// instead of reading env variables directly here — keeps this struct
    /// focused and easier to test.
    pub fn new(smtp_host: &str, smtp_port: u16, smtp_from: &str) -> Self {
        // `builder_dangerous` creates an unencrypted (plain) SMTP connection.
        // This is intentional and SAFE for development — MailHog does not use TLS.
        // Do NOT use this in production!
        let mailer = SmtpTransport::builder_dangerous(smtp_host)
            .port(smtp_port)
            .build();

        Self {
            mailer,
            from: smtp_from.to_string(),
        }
    }
}

impl EmailSender for MailhogEmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        // Build the email with an explicit `text/html; charset=utf-8` content type.
        //
        // WHY: lettre's `.body(String)` defaults to `text/plain`, which causes
        // two problems in MailHog:
        //   1. HTML tags are shown as raw text instead of being rendered.
        //   2. The body gets Quoted-Printable encoded, turning `=` into `=3D`,
        //      `>` into surrounding noise, etc.
        //
        // Fix: use `SinglePart::builder()` with `ContentType::TEXT_HTML` so
        // lettre knows to set `Content-Type: text/html; charset=utf-8` and
        // encode it correctly as UTF-8, not Quoted-Printable.
        let email = Message::builder()
            .from(self.from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body.to_string()),
            )?;

        // Send the email via the SMTP transport
        self.mailer.send(&email)?;

        println!("Email successfully sent to: {to}");
        Ok(())
    }
}