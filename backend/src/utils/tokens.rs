use crate::utils::salted_hashes::generate_b64_hash_for_text_and_salt;
use base64::engine::{general_purpose, GeneralPurpose};
use base64::{DecodeError, Engine};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::string::FromUtf8Error;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

const BASE_64: GeneralPurpose = general_purpose::STANDARD;

#[derive(Debug)]
pub enum CreateAccessTokenError {
    JsonError(serde_json::Error),
    DecodeError(DecodeError),
}

#[derive(Debug)]
pub enum ParseAccessTokenError {
    ParseJsonError(serde_json::Error),
    CreateAccessTokenError(CreateAccessTokenError),
    InvalidDigest,
    Base64DecodeError(DecodeError),
    StringDecodingError(FromUtf8Error),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    pub token: String,
    pub expires_at: PrimitiveDateTime,
    pub refresh_at: PrimitiveDateTime,
}

impl AccessTokenResponse {
    pub fn new(user: UserInfo) -> Result<Self, CreateAccessTokenError> {
        let access_token = AccessToken::new_with_user(user);

        let digest_access_token: DigestAccessToken = access_token.try_into()?;

        digest_access_token.try_into()
    }
}

impl TryFrom<DigestAccessToken> for AccessTokenResponse {
    type Error = CreateAccessTokenError;

    fn try_from(value: DigestAccessToken) -> Result<Self, Self::Error> {
        let token = serde_json::to_string(&value).map_err(CreateAccessTokenError::JsonError)?;

        Ok(AccessTokenResponse {
            token: BASE_64.encode(token),
            expires_at: value.expires_at,
            refresh_at: value.refresh_at,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    user: UserInfo,
    expires_at: PrimitiveDateTime,
    refresh_at: PrimitiveDateTime,
}

impl AccessToken {
    pub fn new_with_user(user: UserInfo) -> Self {
        let duration_in_secs = std::env::var("TOKEN_DURATION_IN_SECS")
            .expect("SECRET_SALT must be in environment")
            .parse::<i64>()
            .expect("integer duration");
        let duration = Duration::seconds(duration_in_secs);

        Self::new_with_user_and_duration(user, duration)
    }

    fn new_with_user_and_duration(user: UserInfo, duration: Duration) -> Self {
        let expires_at = OffsetDateTime::now_utc().add(duration);
        let expires_at = PrimitiveDateTime::new(expires_at.date(), expires_at.time());

        let refresh_at: OffsetDateTime = OffsetDateTime::now_utc().add(duration * 2);
        let refresh_at = PrimitiveDateTime::new(refresh_at.date(), refresh_at.time());

        Self {
            user,
            expires_at,
            refresh_at,
        }
    }

    pub fn from_token<T: AsRef<str>>(token_b64: T) -> Result<Self, ParseAccessTokenError> {
        let token_bytes = BASE_64
            .decode(token_b64.as_ref())
            .map_err(ParseAccessTokenError::Base64DecodeError)?;

        let token =
            String::from_utf8(token_bytes).map_err(ParseAccessTokenError::StringDecodingError)?;

        let digest_token: DigestAccessToken =
            serde_json::from_str(&token).map_err(ParseAccessTokenError::ParseJsonError)?;

        let access_token: AccessToken = serde_json::from_str(digest_token.token.as_ref())
            .map_err(ParseAccessTokenError::ParseJsonError)?;
        let user = access_token.user.clone();
        let expires_at = access_token.expires_at;
        let refresh_at = access_token.refresh_at;

        let given_digest_access_token: DigestAccessToken = access_token
            .try_into()
            .map_err(ParseAccessTokenError::CreateAccessTokenError)?;

        if given_digest_access_token.digest != digest_token.digest {
            return Err(ParseAccessTokenError::InvalidDigest);
        }

        Ok(AccessToken {
            user,
            expires_at,
            refresh_at,
        })
    }

    pub fn get_user(&self) -> &UserInfo {
        &self.user
    }

    pub fn get_expires_at(&self) -> &PrimitiveDateTime {
        &self.expires_at
    }
}

#[derive(Serialize, Deserialize)]
pub struct DigestAccessToken {
    digest: String,
    token: String,
    expires_at: PrimitiveDateTime,
    refresh_at: PrimitiveDateTime,
}

impl TryFrom<AccessToken> for DigestAccessToken {
    type Error = CreateAccessTokenError;

    fn try_from(value: AccessToken) -> Result<Self, Self::Error> {
        let token = serde_json::to_string(&value).map_err(CreateAccessTokenError::JsonError)?;

        let salt_b64 = std::env::var("SECRET_SALT").expect("SECRET_SALT must be in environment");

        let digest = generate_b64_hash_for_text_and_salt(&token, salt_b64)
            .map_err(CreateAccessTokenError::DecodeError)?;

        Ok(DigestAccessToken {
            digest,
            token,
            expires_at: value.expires_at,
            refresh_at: value.refresh_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tokens::AccessTokenResponse;
    use database::utils::random_samples::RandomSample;
    use dotenvy::dotenv;

    #[test]
    fn test_create_and_parse_a_valid_token() {
        dotenv().expect("failed to load .env");

        let user_id = Uuid::new_v4();
        let first_name = Some(String::new_random(124));
        let last_name = Some(String::new_random(124));

        let user = UserInfo {
            user_id,
            first_name,
            last_name,
        };

        let token_response = AccessTokenResponse::new(user.clone()).expect("valid token");

        let token = token_response.token;

        let parsed_access_token = AccessToken::from_token(token).expect("valid token");

        assert_eq!(parsed_access_token.user, user);
        assert_eq!(parsed_access_token.expires_at, token_response.expires_at);
        assert_eq!(parsed_access_token.refresh_at, token_response.refresh_at);
    }

    #[test]
    fn test_parse_invalid_token() {
        dotenv().expect("failed to load .env");

        let user_id = Uuid::new_v4();
        let first_name = Some(String::new_random(124));
        let last_name = Some(String::new_random(124));

        let user = UserInfo {
            user_id,
            first_name,
            last_name,
        };

        let access_token = AccessToken::new_with_user(user);
        let expires_at = access_token.expires_at;
        let refresh_at = access_token.refresh_at;

        let digest_access_token = DigestAccessToken {
            digest: String::new_random(88),
            token: serde_json::to_string(&access_token)
                .map_err(CreateAccessTokenError::JsonError)
                .expect("TODO: valid access token"),
            expires_at,
            refresh_at,
        };

        let token_response: AccessTokenResponse = digest_access_token.try_into().expect("TODO");

        let token = token_response.token;

        let parsed_access_token_or_error = AccessToken::from_token(token);
        assert!(parsed_access_token_or_error.is_err())
    }
}
