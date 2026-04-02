use actix_web::{App, HttpServer};
use crate::infrastructure::config::Config;

mod domain;
mod infrastructure;
mod startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load config once here just to print startup info
    let config = Config::load();

    let host = config.app_host.clone();
    let port = config.app_port;

    println!("🚀 Starting Paybridge Notification Service");
    println!("📡 Server running at http://{host}:{port}");
    println!("📬 SMTP target: {}:{}", config.smtp_host, config.smtp_port);
    println!("📮 MailHog Web UI: http://localhost:8025");

    HttpServer::new(|| {
        App::new()
            // configure_notification will:
            // - load SMTP config
            // - create MailhogEmailSender
            // - create NotificationService
            // - register all routes
            .configure(startup::configure_notification)
    })
        .bind((host, port))?
        .run()
        .await
}