use axum::http::header;
use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;

use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod auth;
mod config;
mod db;
mod email;
mod errors;
mod handlers;
mod models;
use config::Config;
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub config: Config,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sharplines_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let config = Config::from_env();
    let pool = db::init_pool(&config.database_url).await;

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::COOKIE])
        .allow_credentials(true);
    let state = AppState { pool, config };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/refresh", post(handlers::auth::refresh))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/auth/verify-email", get(handlers::auth::verify_email))
        .route(
            "/auth/resend-verification",
            post(handlers::auth::resend_verification),
        )
        .route(
            "/auth/forgot-password",
            post(handlers::auth::forgot_password),
        )
        .route("/auth/reset-password", post(handlers::auth::reset_password))
        .route("/auth/lichess", get(auth::lichess::login_lichess))
        .route(
            "/auth/lichess/callback",
            get(auth::lichess::lichess_callback),
        )
        .route("/auth/me", get(handlers::auth::me))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    async fn health_check() -> &'static str {
        "OK"
    }
}
