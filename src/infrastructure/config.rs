use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    // Database connection string — optional for now.
    // Will be required once DB features are implemented.
    pub database_url: Option<String>,

    // Server host and port
    pub app_host: String,
    pub app_port: u16,

    // SMTP config for MailHog
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_from: String,
}

impl Config {
    pub fn load() -> Self {
        // Load variables from the `.env` file into the environment.
        // If the file doesn't exist (e.g. in production), that's fine.
        dotenv().ok();

        // DATABASE_URL is optional for now.
        // Using `ok()` so the app won't crash if it's not set yet.
        let database_url = env::var("DATABASE_URL").ok();

        // APP_HOST: defaults to "0.0.0.0" so the service is reachable
        // from outside the container.
        let app_host = env::var("APP_HOST")
            .unwrap_or_else(|_| "0.0.0.0".into());

        // APP_PORT: defaults to 8081 if not set.
        let app_port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8081".into())
            .parse::<u16>()
            .expect("APP_PORT must be a valid number (0-65535)");

        // SMTP_HOST: the hostname of the MailHog server.
        // - When running locally (cargo run): "localhost"
        // - When running inside Docker: use the service name, e.g. "mailhog"
        let smtp_host = env::var("SMTP_HOST")
            .unwrap_or_else(|_| "localhost".into());

        // SMTP_PORT: MailHog's SMTP port, defaults to 1025.
        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "1025".into())
            .parse::<u16>()
            .expect("SMTP_PORT must be a valid number");

        // SMTP_FROM: the sender address shown on outgoing emails.
        let smtp_from = env::var("SMTP_FROM")
            .unwrap_or_else(|_| "no-reply@paybridge.local".into());

        Self {
            database_url,
            app_host,
            app_port,
            smtp_host,
            smtp_port,
            smtp_from,
        }
    }
}