use crate::AppState;
use crate::auth::jwt::{generate_access_token, generate_refresh_token};
use crate::models::user::{User, UserResponse};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use serde_json::json;
use sha2::Digest;

#[derive(Deserialize)]
pub struct LichessCallbackParams {
    pub code: Option<String>,
    pub error: Option<String>,
}
pub async fn login_lichess(State(state): State<AppState>) -> impl IntoResponse {
    let authorize_url = format!(
        "https://lichess.org/oauth?response_type=code&client_id={}&redirect_uri={}&scope=preference:read",
        state.config.lichess_client_id, state.config.lichess_redirect_uri,
    );
    Redirect::to(&authorize_url)
}
pub async fn lichess_callback(
    State(state): State<AppState>,
    Query(params): Query<LichessCallbackParams>,
) -> impl IntoResponse {
    if let Some(_err) = params.error {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Lichess authorization denied"})),
        )
            .into_response();
    }
    let code = match params.code {
        Some(c) => c,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing authorization code"})),
            )
                .into_response();
        }
    };
    // Exchange code for token
    let client = reqwest::Client::new();
    let token_response = client
        .post("https://lichess.org/api/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", &state.config.lichess_redirect_uri),
            ("client_id", &state.config.lichess_client_id),
            ("client_secret", &state.config.lichess_client_secret),
        ])
        .send()
        .await;
    let token_data = match token_response {
        Ok(r) if r.status().is_success() => r.json::<serde_json::Value>().await.unwrap_or_default(),
        Ok(r) => {
            let status = r.status();
            let body = r.text().await.unwrap_or_default();
            return (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Lichess token exchange failed ({}): {}", status, body)}))).into_response();
        }
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Failed to contact Lichess"})),
            )
                .into_response();
        }
    };
    let access_token = match token_data.get("access_token").and_then(|v| v.as_str()) {
        Some(t) => t.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "No access token in Lichess response"})),
            )
                .into_response();
        }
    };
    // Fetch user profile
    let profile = client
        .get("https://lichess.org/api/account")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await;
    let profile_data = match profile {
        Ok(r) if r.status().is_success() => r.json::<serde_json::Value>().await.unwrap_or_default(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Failed to fetch Lichess profile"})),
            )
                .into_response();
        }
    };
    let lichess_id = match profile_data.get("id").and_then(|v| v.as_str()) {
        Some(i) => i.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid Lichess profile"})),
            )
                .into_response();
        }
    };
    let email = profile_data
        .get("email")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let display_name = profile_data
        .get("username")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    // Find or create user
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE lichess_user_id = $1")
        .bind(&lichess_id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(u)) => u,
        Ok(None) => {
            // Create new user
            match sqlx::query_as::<_, User>(
                "INSERT INTO users (email, display_name, lichess_user_id, lichess_access_token, email_verified) \
                 VALUES ($1, $2, $3, $4, TRUE) RETURNING *"
            )
            .bind(&email)
            .bind(&display_name)
            .bind(&lichess_id)
            .bind(&access_token)
            .fetch_one(&state.pool)
            .await
            {
                Ok(u) => u,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create user"}))).into_response(),
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
                .into_response();
        }
    };
    // Issue tokens
    let jwt_token = match generate_access_token(user.id, &state.config.jwt_secret) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to generate token"})),
            )
                .into_response();
        }
    };
    let refresh_token = generate_refresh_token();
    let token_hash = format!("{:x}", sha2::Sha256::digest(refresh_token.as_bytes()));
    let expires_at =
        chrono::Utc::now() + chrono::Duration::days(state.config.jwt_refresh_expiry_days);
    let _ = sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind(&token_hash)
    .bind(expires_at)
    .execute(&state.pool)
    .await;
    // Redirect to frontend with cookies
    let frontend_url = &state.config.frontend_url;
    let body = serde_json::to_string(&json!({"message": "Login successful"})).unwrap();
    axum::response::Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", frontend_url.as_str())
        .header(
            "Set-Cookie",
            format!(
                "refresh_token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax",
                refresh_token,
                state.config.jwt_refresh_expiry_days * 86400
            ),
        )
        .body(axum::body::Body::from(body))
        .unwrap()
}
