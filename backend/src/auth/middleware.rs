use crate::AppState;

use super::jwt::{Claims, verify_access_token};
use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

pub struct AuthUser {
    pub user_id: Uuid,
}

pub struct MayBeAuthUser(pub Option<Uuid>);

pub struct AuthError;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Unauthorized", "status": 401})),
        )
            .into_response()
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError)?;
        let token = auth_header.strip_prefix("Bearer ").ok_or(AuthError)?;
        let claims: Claims =
            verify_access_token(token, &state.config.jwt_secret).map_err(|_| AuthError)?;
        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
