use actix_web::web;
use std::sync::Arc;

use crate::domain::notification::{
    routes::{test_email, registration_email},
    service::NotificationService,
};
use crate::infrastructure::{
    config::Config,
    mail::mailhog::MailhogEmailSender,
};

/// Registers all routes and dependencies needed by the notification module.
/// Called from main.rs via `App::configure()`.
///
/// Dependency injection flow:
/// Config → MailhogEmailSender → NotificationService → Route Handler
pub fn configure_notification(cfg: &mut web::ServiceConfig) {
    // 1. Load config from .env / environment variables
    let config = Config::load();

    // 2. Create the email sender using SMTP settings from Config.
    //    Arc<dyn EmailSender> = a reference-counted pointer that can be
    //    safely shared across threads (required by Actix).
    let email_sender = Arc::new(MailhogEmailSender::new(
        &config.smtp_host,
        config.smtp_port,
        &config.smtp_from,
    ));

    // 3. Inject the email sender into NotificationService
    let notification_service = NotificationService::new(email_sender);

    // 4. Register the service as app_data and mount the routes
    cfg
        .app_data(web::Data::new(notification_service))
        .service(test_email)
        .service(registration_email);
}