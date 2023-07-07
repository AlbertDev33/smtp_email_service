use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from: String,
    pub smtp_to: String,
}

impl Config {
    pub fn init() -> Config {
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_port = env::var("SMTP_PORT").expect("SMTP_PORT must be set");
        let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        let smtp_from = env::var("SMTP_FROM").expect("SMTP_FROM must be set");
        let smtp_to = env::var("SMTP_TO").expect("SMTP_TO must be set");

        return Config {
            smtp_host,
            smtp_port: smtp_port.parse::<u16>().unwrap(),
            smtp_user,
            smtp_pass,
            smtp_from,
            smtp_to,
        };
    }
}
