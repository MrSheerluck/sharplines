use crate::AppState;
use crate::auth::jwt::{generate_access_token, generate_refresh_token};
use crate::auth::password::{hash_password, verify_password};
use axum::body::Body;
use axum::response::Response;

use crate::email::resend::{send_email, verification_email};
use crate::models::user::{CreateUserRequest, LoginRequest, User, UserResponse};

use crate::auth::middleware::AuthUser;
use crate::email::resend::password_reset_email;
use axum::http::{HeaderMap, StatusCode};
use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;
use sha2::Digest;
use uuid::Uuid;

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> impl IntoResponse {
    if body.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Password must be at least 8 characters"})),
        )
            .into_response();
    }
    let password_hash = match hash_password(&body.password) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to hash password"})),
            )
                .into_response();
        }
    };
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, display_name) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&body.email)
    .bind(&password_hash)
    .bind(&body.display_name)
    .fetch_one(&state.pool)
    .await;

    let user = match result {
        Ok(u) => u,
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            return (
                StatusCode::CONFLICT,
                Json(json!({"error": "Email already exists"})),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Registration failed"})),
            )
                .into_response();
        }
    };
    let verification_token = Uuid::new_v4().to_string();
    let _ = sqlx::query("UPDATE users SET email_verification_token = $1, email_verification_sent_at = NOW() WHERE id = $2")
            .bind(&verification_token)
            .bind(user.id)
            .execute(&state.pool)
            .await;

    let frontend_url = &state.config.frontend_url;
    let verify_url = format!("{}/verify-email?token={}", frontend_url, verification_token);
    let (html, text) = verification_email(&verify_url);
    if let Err(e) = send_email(
        &state.config.smtp_host,
        state.config.smtp_port,
        &state.config.smtp_username,
        &state.config.smtp_password,
        &state.config.email_from_address,
        &body.email,
        "Verify your email for SharpLines",
        &html,
        &text,
    )
    .await
    {
        tracing::error!("Failed to send verification email: {}", e);
    }
    (
        StatusCode::CREATED,
        Json(json!({"message": "Check your email to verify your account"})),
    )
        .into_response()
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid email or password"})),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Login failed"})),
            )
                .into_response();
        }
    };
    let valid = match &user.password_hash {
        Some(hash) => verify_password(&body.password, hash).unwrap_or(false),
        None => false,
    };
    if !valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid email or password"})),
        )
            .into_response();
    }
    if !user.email_verified {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Please verify your email before logging in", "code": "EMAIL_NOT_VERIFIED"})),
        )
            .into_response();
    }
    let access_token = match generate_access_token(user.id, &state.config.jwt_secret) {
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
    let token_hash = sha2::Sha256::digest(refresh_token.as_bytes());
    let token_hash_hex = format!("{:x}", token_hash);
    let expires_at =
        chrono::Utc::now() + chrono::Duration::days(state.config.jwt_refresh_expiry_days);
    let _ = sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind(&token_hash_hex)
    .bind(expires_at)
    .execute(&state.pool)
    .await;
    let response: UserResponse = user.into();
    let body = serde_json::to_string(&json!({
        "access_token": access_token,
        "token_type": "Bearer",
        "expires_in": 900,
        "user": response,
    }))
    .unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header(
            "Set-Cookie",
            format!(
                "refresh_token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax",
                refresh_token,
                state.config.jwt_refresh_expiry_days * 86400
            ),
        )
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap()
}

pub async fn me(State(state): State<AppState>, auth: AuthUser) -> impl IntoResponse {
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth.user_id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(user)) => {
            let response: UserResponse = user.into();
            (StatusCode::OK, Json(json!(response))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch user"})),
        )
            .into_response(),
    }
}

pub async fn refresh(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let cookie = match headers.get("Cookie").and_then(|v| v.to_str().ok()) {
        Some(c) => c,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "No refresh token"})),
            )
                .into_response();
        }
    };
    let token = cookie
        .split(';')
        .filter_map(|part| {
            let trimmed = part.trim();
            trimmed.strip_prefix("refresh_token=")
        })
        .next()
        .map(|t| t.to_string());
    let token = match token {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "No refresh token"})),
            )
                .into_response();
        }
    };
    let token_hash = format!("{:x}", sha2::Sha256::digest(token.as_bytes()));
    let stored = match sqlx::query_as::<sqlx::Postgres, (String, Uuid, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>)>(
            "SELECT token_hash, user_id, expires_at, revoked_at FROM refresh_tokens WHERE token_hash = $1",
        )
    .bind(&token_hash)
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(s)) => s,
        Ok(None) => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid refresh token"}))).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Token validation failed"}))).into_response(),
    };
    if stored.3.is_some() || stored.2 < chrono::Utc::now() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Refresh token expired or revoked"})),
        )
            .into_response();
    }
    let user_id = stored.1;
    let new_access_token = match generate_access_token(user_id, &state.config.jwt_secret) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to generate token"})),
            )
                .into_response();
        }
    };
    let new_refresh_token = generate_refresh_token();
    let new_token_hash = format!("{:x}", sha2::Sha256::digest(new_refresh_token.as_bytes()));
    let expires_at =
        chrono::Utc::now() + chrono::Duration::days(state.config.jwt_refresh_expiry_days);
    let _ = sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(&state.pool)
        .await;
    let _ = sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user_id)
    .bind(&new_token_hash)
    .bind(expires_at)
    .execute(&state.pool)
    .await;
    let body = serde_json::to_string(&json!({"access_token": new_access_token})).unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header(
            "Set-Cookie",
            format!(
                "refresh_token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax",
                new_refresh_token,
                state.config.jwt_refresh_expiry_days * 86400
            ),
        )
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap()
}

pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let token = headers
        .get("Cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|cookie| {
            cookie
                .split(';')
                .filter_map(|part| part.trim().strip_prefix("refresh_token="))
                .next()
                .map(|t| t.to_string())
        });
    if let Some(ref token) = token {
        let token_hash = format!("{:x}", sha2::Sha256::digest(token.as_bytes()));
        let _ = sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1")
            .bind(&token_hash)
            .execute(&state.pool)
            .await;
    }
    Response::builder()
        .status(StatusCode::OK)
        .header(
            "Set-Cookie",
            "refresh_token=; HttpOnly; Path=/; Max-Age=0; SameSite=Lax",
        )
        .header("Content-Type", "application/json")
        .body(Body::from("{}"))
        .unwrap()
}

pub async fn verify_email(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match params.get("token") {
        Some(t) => t,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing verification token"})),
            )
                .into_response();
        }
    };
    let result =
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email_verification_token = $1")
            .bind(token)
            .fetch_optional(&state.pool)
            .await;
    let user = match result {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid verification token"})),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Verification failed"})),
            )
                .into_response();
        }
    };
    let _ = sqlx::query(
        "UPDATE users SET email_verified = TRUE, email_verification_token = NULL WHERE id = $1",
    )
    .bind(user.id)
    .execute(&state.pool)
    .await;
    (
        StatusCode::OK,
        Json(json!({"message": "Email verified successfully"})),
    )
        .into_response()
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let email = match body.get("email").and_then(|v| v.as_str()) {
        Some(e) => e,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Email is required"})),
            )
                .into_response();
        }
    };
    // Always return success to avoid leaking user existence
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(&state.pool)
        .await;
    if let Ok(Some(user)) = user {
        let reset_token = Uuid::new_v4().to_string();
        let token_hash = format!("{:x}", sha2::Sha256::digest(reset_token.as_bytes()));
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);
        let _ = sqlx::query(
            "INSERT INTO password_resets (user_id, token_hash, expires_at) VALUES ($1, $2, $3) \
             ON CONFLICT (token_hash) DO UPDATE SET token_hash = EXCLUDED.token_hash, expires_at = EXCLUDED.expires_at",
        )
        .bind(user.id)
        .bind(&token_hash)
        .bind(expires_at)
        .execute(&state.pool)
        .await;
        let reset_url = format!(
            "{}/reset-password?token={}",
            state.config.frontend_url, reset_token
        );
        let (html, text) = password_reset_email(&reset_url);
        let _ = send_email(
            &state.config.smtp_host,
            state.config.smtp_port,
            &state.config.smtp_username,
            &state.config.smtp_password,
            &state.config.email_from_address,
            email,
            "Reset your SharpLines password",
            &html,
            &text,
        )
        .await;
    }
    (
        StatusCode::OK,
        Json(json!({"message": "Check your email for the reset link"})),
    )
        .into_response()
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let token = match body.get("token").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing reset token"})),
            )
                .into_response();
        }
    };
    let new_password = match body.get("password").and_then(|v| v.as_str()) {
        Some(p) if p.len() >= 8 => p,
        Some(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Password must be at least 8 characters"})),
            )
                .into_response();
        }
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Password is required"})),
            )
                .into_response();
        }
    };
    let token_hash = format!("{:x}", sha2::Sha256::digest(token.as_bytes()));
    let reset = match sqlx::query_as::<sqlx::Postgres, (String, Uuid, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT token_hash, user_id, expires_at, used_at FROM password_resets WHERE token_hash = $1",
    )
    .bind(&token_hash)
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid reset token"}))).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Password reset failed"}))).into_response(),
    };
    if reset.3.is_some() || reset.2 < chrono::Utc::now() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Reset token expired or already used"})),
        )
            .into_response();
    }
    let password_hash = match hash_password(new_password) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to hash password"})),
            )
                .into_response();
        }
    };
    let _ = sqlx::query("UPDATE password_resets SET used_at = NOW() WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(&state.pool)
        .await;
    let _ = sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
        .bind(&password_hash)
        .bind(reset.1)
        .execute(&state.pool)
        .await;
    let _ = sqlx::query(
        "UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(reset.1)
    .execute(&state.pool)
    .await;
    (
        StatusCode::OK,
        Json(json!({"message": "Password updated successfully"})),
    )
        .into_response()
}

pub async fn resend_verification(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let email = match body.get("email").and_then(|v| v.as_str()) {
        Some(e) => e,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Email is required"})),
            )
                .into_response();
        }
    };
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::OK,
                Json(json!({"message": "Check your email to verify your account"})),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to resend verification"})),
            )
                .into_response();
        }
    };
    if user.email_verified {
        return (
            StatusCode::OK,
            Json(json!({"message": "Email already verified"})),
        )
            .into_response();
    }
    let verification_token = Uuid::new_v4().to_string();
    let _ = sqlx::query("UPDATE users SET email_verification_token = $1, email_verification_sent_at = NOW() WHERE id = $2")
        .bind(&verification_token)
        .bind(user.id)
        .execute(&state.pool)
        .await;
    let verify_url = format!(
        "{}/verify-email?token={}",
        state.config.frontend_url, verification_token
    );
    let (html, text) = verification_email(&verify_url);
    if let Err(e) = send_email(
        &state.config.smtp_host,
        state.config.smtp_port,
        &state.config.smtp_username,
        &state.config.smtp_password,
        &state.config.email_from_address,
        email,
        "subject",
        &html,
        &text,
    )
    .await
    {
        tracing::error!("Failed to send verification email: {}", e);
    }
    (
        StatusCode::OK,
        Json(json!({"message": "Check your email to verify your account"})),
    )
        .into_response()
}
