use std::sync::Arc;
use anyhow::Result;
use crate::domain::notification::port::EmailSender;
use crate::domain::notification::dto::RegistrationEmailRequest;
use crate::domain::notification::templates;
pub struct NotificationService {
    email_sender: Arc<dyn EmailSender>,
}

impl NotificationService {
    pub fn new(email_sender: Arc<dyn EmailSender>) -> Self {
        Self { email_sender }
    }

    pub fn send_test_email(&self, to: &str) -> Result<()> {
        println!("Sending test email to {}", to);
        self.email_sender.send(
            to,
            "Paybridge Notification Test",
            "This is a dummy MailHog test email",
        )
    }

    /// Sends a registration success email with an HTML body.
    ///
    /// Accepts a `RegistrationEmailRequest` which carries:
    /// - `to`                — recipient email address
    /// - `name`              — user's display name (injected into the template)
    /// - `verification_link` — the link the user clicks to verify their account
    pub fn send_registration_email(&self, req: &RegistrationEmailRequest) -> Result<()> {
        // Build the HTML body from the template, injecting user-specific values
        let html_body = templates::registration_success(&req.name, &req.verification_link);

        self.email_sender.send(
            &req.to,
            "Welcome to Paybridge — Please Verify Your Email",
            &html_body,
        )
    }
}

