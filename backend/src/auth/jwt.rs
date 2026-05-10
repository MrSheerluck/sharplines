use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

pub fn generate_access_token(
    user_id: Uuid,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(15)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_access_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub fn generate_refresh_token() -> String {
    use rand::Rng;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill(&mut bytes);
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    #[test]
    fn test_access_token_round_trip() {
        let user_id = Uuid::new_v4();
        let secret = "test-secret-at-least-32-chars-long!!";
        let token = generate_access_token(user_id, secret).unwrap();
        let claims = verify_access_token(&token, secret).unwrap();
        assert_eq!(claims.sub, user_id);
    }
    #[test]
    fn test_expired_token_fails() {
        let user_id = Uuid::new_v4();
        let secret = "test-secret-at-least-32-chars-long!!";
        let mut claims = Claims {
            sub: user_id,
            iat: 0,
            exp: 0, // expired in 1970
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();
        assert!(verify_access_token(&token, secret).is_err());
    }
    #[test]
    fn test_tampered_token_fails() {
        let user_id = Uuid::new_v4();
        let secret = "test-secret-at-least-32-chars-long!!";
        let token = generate_access_token(user_id, secret).unwrap();
        let mut bytes = token.into_bytes();
        if let Some(b) = bytes.last_mut() {
            *b ^= 0x01; // flip a bit
        }
        let tampered = String::from_utf8(bytes).unwrap();
        assert!(verify_access_token(&tampered, secret).is_err());
    }
}
