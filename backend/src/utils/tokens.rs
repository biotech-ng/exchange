use crate::utils::salted_hashes::generate_b64_hash_for_text_and_salt;
use base64::engine::{general_purpose, GeneralPurpose};
use base64::{DecodeError, Engine};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::string::FromUtf8Error;
use time::{Duration, OffsetDateTime};
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

#[derive(Debug, Clone)]
pub struct AccessTokenResponse {
    pub token: String,
    pub expires_at: OffsetDateTime,
    pub refresh_at: OffsetDateTime,
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
    expires_at: OffsetDateTime,
    refresh_at: OffsetDateTime,
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

        let refresh_at: OffsetDateTime = OffsetDateTime::now_utc().add(duration * 2);

        Self {
            user,
            expires_at,
            refresh_at,
        }
    }

    pub fn from_token(token_b64: impl AsRef<str>) -> Result<Self, ParseAccessTokenError> {
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

    pub fn get_expires_at(&self) -> &OffsetDateTime {
        &self.expires_at
    }

    pub fn get_refresh_at(&self) -> &OffsetDateTime {
        &self.refresh_at
    }
}

#[derive(Serialize, Deserialize)]
pub struct DigestAccessToken {
    digest: String,
    token: String,
    expires_at: OffsetDateTime,
    refresh_at: OffsetDateTime,
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
pub mod tests {
    use super::*;
    use crate::utils::tokens::AccessTokenResponse;
    use database::utils::random_samples::RandomSample;
    use dotenvy::dotenv;

    pub fn create_token() -> (UserInfo, AccessTokenResponse) {
        dotenv().expect("failed to load .env");

        let user_id = Uuid::new_v4();
        let first_name = Some(String::new_random(124));
        let last_name = Some(String::new_random(124));

        let user = UserInfo {
            user_id,
            first_name,
            last_name,
        };

        (
            user.clone(),
            AccessTokenResponse::new(user).expect("valid token"),
        )
    }

    pub fn make_expired_token(user: UserInfo, minus_duration: Duration) -> AccessTokenResponse {
        // past
        let expires_at = OffsetDateTime::now_utc().add(-minus_duration);

        // distant future
        let refresh_at: OffsetDateTime = OffsetDateTime::now_utc().add(Duration::days(1000));

        let access_token = AccessToken {
            user,
            expires_at,
            refresh_at,
        };

        let digest_access_token: DigestAccessToken =
            access_token.try_into().expect("valid digest token");

        digest_access_token.try_into().expect("valid token")
    }

    #[test]
    fn test_create_and_parse_a_valid_token() {
        let (user, token_response) = create_token();

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
                .expect("valid access token"),
            expires_at,
            refresh_at,
        };

        let token_response: AccessTokenResponse =
            digest_access_token.try_into().expect("valid token");

        let token = token_response.token;

        let parsed_access_token_or_error = AccessToken::from_token(token);
        assert!(parsed_access_token_or_error.is_err())
    }
}
