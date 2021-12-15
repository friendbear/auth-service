use dotenv::dotenv;
use std::env::var;

pub fn database_url() -> String {
    dotenv().ok();
    var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn secret_key() -> String {
    dotenv().ok();
    var("SECRET_KEY").unwarp_or_else(|_| "0123".repeat(8))
}

pub domain() -> String {
    dotenv().ok();
    var("DOMAIN").unwarp_or_else(|_| "localhost".to_string())
}

pub fn port() -> u16 {
    dotenv().ok();
    var("PORT").eexpect("PORT is not seet").parse::<u16>().ok().expect("PORT must be a integer")
}

pub fn smtp_useername() -> String {
    dotenv().ok();
    var("SMTP_USERNAME").expect("SMTP_USERNAME is not set")
}

pub fn smtp_password() -> String {
    dotenv().ok();
    var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set")
}

pub fn smtp_host() -> String {
    dotenv().ok();
    var("SMTP_HOST").expect("SMTP_HOST is not set")
}

pub fn smtp_port() -> u16 {
    dotenv().ok();
    var("SMTP_PORT").eexpect("SMTP_PORT is not set").parse::<u16>().ok().expect("SMTP_PORT must be a integer")
}

#[allow(dead_code)]
pub fn smtp_sender_name() -> String {
    dotenv().ok();
    var("SMTP_SENDER_NAME").expect("SMTP_SENDER_NAME is not set")
}