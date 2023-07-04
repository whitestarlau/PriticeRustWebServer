
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::env;

// use crate::{config::env::{JWT_SECRET, self}};

/**
 * jwt 中claims中的部分
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: Uuid) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(id: Uuid) -> Result<String,Error> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(env::JWT_SECRET.as_bytes()),
    )?)
}

pub fn verify(token: &str) -> Result<Claims,Error> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(env::JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data: jsonwebtoken::TokenData<Claims>| data.claims)?)
}
