use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,

    pub jwt_secret: String,
    pub jwt_access_expiry_minutes: i64,
    pub jwt_refresh_expiry_days: i64,

    pub backend_port: u16,
    pub frontend_url: String,

    pub encryption_key: String,

    pub resend_api_key: String,
    pub email_from_address: String,
    pub email_from_name: String,

    pub lichess_client_id: String,
    pub lichess_client_secret: String,
    pub lichess_redirect_uri: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: get_env("DATABASE_URL"),
            jwt_secret: get_env("JWT_SECRET"),
            jwt_access_expiry_minutes: get_env_parse("JWT_ACCESS_EXPIRY"),
            jwt_refresh_expiry_days: get_env_parse("JWT_REFRESH_EXPIRY"),
            backend_port: get_env_parse("BACKEND_PORT"),
            frontend_url: get_env("FRONTEND_URL"),
            encryption_key: get_env("ENCRYPTION_KEY"),
            resend_api_key: get_env("RESEND_API_KEY"),
            email_from_address: get_env("EMAIL_FROM_ADDRESS"),
            email_from_name: get_env("EMAIL_FROM_NAME"),
            lichess_client_id: get_env("LICHESS_CLIENT_ID"),
            lichess_client_secret: get_env("LICHESS_CLIENT_SECRET"),
            lichess_redirect_uri: get_env("LICHESS_REDIRECT_URI"),
        }
    }
}

fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}
fn get_env_parse<T: std::str::FromStr>(key: &str) -> T {
    env::var(key)
        .unwrap_or_else(|_| panic!("{} must be set", key))
        .parse()
        .unwrap_or_else(|_| panic!("{} must be a valid number", key))
}
