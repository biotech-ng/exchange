use crate::errors::errors::DbError;
use crate::models::project::ProjectDb;
use crate::models::user::UserDb;
use crate::utils::tokens::{AccessToken, AccessTokenResponse, DigestAccessToken, UserInfo};
use crate::web::errors::{
    INTERNAL_SERVER_ERROR_RESPONSE, INVALID_TOKEN_FORMAT_ERROR_MSG, UNAUTHORIZED_ERROR_MSG,
};
use crate::web_service::{ErrorResponseBody, WebService};
use axum::extract::{FromRequestParts, State};
use axum::http::{HeaderMap, HeaderValue, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_auth::AuthBearer;
use hyper::StatusCode;
use std::ops::Add;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

#[derive(Debug)]
pub enum AuthenticationError {
    InvalidInputTokenFormat,
    InvalidTokenFormatInDb,
    DecodeTokenError,
    DbError(DbError),
    Unauthorised,
}

pub trait AuthHeaders {
    fn add_auth_headers(&mut self, token: AccessTokenResponse);
}

impl AuthHeaders for HeaderMap {
    fn add_auth_headers(&mut self, token: AccessTokenResponse) {
        self.insert(
            "x-auth-token",
            HeaderValue::try_from(token.token).expect("TODO"),
        );
        // TODO choose better serialization format
        self.insert(
            "x-auth-token-expires-at",
            HeaderValue::try_from(token.expires_at.to_string()).expect("TODO"),
        );
        self.insert(
            "x-auth-token-refresh-at",
            HeaderValue::try_from(token.refresh_at.to_string()).expect("TODO"),
        );
    }
}

async fn authenticate_with_token(
    token: impl AsRef<str>,
    user_db: &impl UserDb,
) -> Result<(AccessTokenResponse, UserInfo), AuthenticationError> {
    let access_token = AccessToken::from_token(token.as_ref())
        .map_err(|_| AuthenticationError::InvalidInputTokenFormat)?;
    let user_info = access_token.get_user().clone();

    let now = OffsetDateTime::now_utc();
    let now = PrimitiveDateTime::new(now.date(), now.time());

    // When token is fresh
    if access_token.get_expires_at() > &now {
        let access_token_response = AccessTokenResponse {
            token: String::from(token.as_ref()),
            expires_at: *access_token.get_expires_at(),
            refresh_at: *access_token.get_refresh_at(),
        };
        return Ok((access_token_response, user_info));
    }

    let user = user_db
        .get_user(&access_token.get_user().user_id)
        .await
        .map_err(AuthenticationError::DbError)?;

    if user.access_token != token.as_ref() {
        // In case of token refresh race condition, check a previous token
        if let Some(previous_access_token) = user.previous_access_token {
            let two_minutes_before_now = now.add(-Duration::minutes(1));
            let two_minutes_before_now = PrimitiveDateTime::new(
                two_minutes_before_now.date(),
                two_minutes_before_now.time(),
            );

            if previous_access_token == token.as_ref()
                && access_token.get_expires_at() > &two_minutes_before_now
            {
                let access_token = AccessToken::from_token(&user.access_token)
                    .map_err(|_| AuthenticationError::InvalidTokenFormatInDb)?;
                let access_token_response = AccessTokenResponse {
                    token: user.access_token,
                    expires_at: *access_token.get_expires_at(),
                    refresh_at: *access_token.get_refresh_at(),
                };
                return Ok((access_token_response, user_info));
            }
        }

        return Err(AuthenticationError::Unauthorised);
    }

    if access_token.get_refresh_at() <= &now {
        return Err(AuthenticationError::Unauthorised);
    }

    let new_user_info = UserInfo {
        first_name: user.first_name,
        last_name: user.last_name,
        user_id: user.id,
    };
    let new_access_token: DigestAccessToken = AccessToken::new_with_user(new_user_info)
        .try_into()
        .map_err(|_| AuthenticationError::DecodeTokenError)?;

    let new_token_response: AccessTokenResponse = new_access_token
        .try_into()
        .map_err(|_| AuthenticationError::DecodeTokenError)?;

    let updated_tokens_count = user_db
        .update_user_token(&user.id, &new_token_response.token, &user.access_token)
        .await
        .map_err(AuthenticationError::DbError)?;

    let access_token_response = if updated_tokens_count == 0 {
        // In case of token refresh race condition, return token from a database
        let token = user_db
            .get_access_token(&user_info.user_id)
            .await
            .map_err(AuthenticationError::DbError)?;
        let access_token = AccessToken::from_token(token.clone())
            .map_err(|_| AuthenticationError::InvalidTokenFormatInDb)?;
        AccessTokenResponse {
            token,
            expires_at: *access_token.get_expires_at(),
            refresh_at: *access_token.get_refresh_at(),
        }
    } else {
        new_token_response
    };

    Ok((access_token_response, user_info))
}

pub async fn check_and_refresh_auth_token<B, UDB: UserDb, PDB: ProjectDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, Response> {
    let (mut parts, body) = req.into_parts();

    let invalid_token_format_error = (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponseBody {
            code: None,
            error: INVALID_TOKEN_FORMAT_ERROR_MSG.into(),
        }),
    )
        .into_response();

    match AuthBearer::from_request_parts(&mut parts, &()).await {
        Ok(AuthBearer(token)) => match authenticate_with_token(token, &web_service.user_db).await {
            Ok((token_info, _)) => {
                let req = Request::from_parts(parts, body);

                let mut response = next.run(req).await;
                response.headers_mut().add_auth_headers(token_info);
                Ok(response)
            }
            Err(AuthenticationError::Unauthorised) => Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponseBody {
                    code: None,
                    error: UNAUTHORIZED_ERROR_MSG.into(),
                }),
            )
                .into_response()),
            Err(AuthenticationError::DbError(db_error)) => Err(db_error.into_response()),
            Err(AuthenticationError::InvalidInputTokenFormat) => Err(invalid_token_format_error),
            Err(AuthenticationError::DecodeTokenError) => {
                Err(INTERNAL_SERVER_ERROR_RESPONSE.clone().into_response())
            }
            Err(AuthenticationError::InvalidTokenFormatInDb) => {
                Err(INTERNAL_SERVER_ERROR_RESPONSE.clone().into_response())
            }
        },
        Err(_) => Err(invalid_token_format_error),
    }
}

#[cfg(test)]
mod tests {
    use crate::models::user::{OwnedUser, PgUserDb, UserDb};
    use crate::utils::salted_hashes::generate_hash_and_salt_for_text;
    use crate::utils::tokens::tests::{create_token, make_expired_token};
    use crate::utils::tokens::{AccessTokenResponse, UserInfo};
    use crate::web::authentication::authenticate_with_token;
    use database::utils::random_samples::RandomSample;
    use time::Duration;
    use uuid::Uuid;

    async fn create_user(
        user_db: &impl UserDb,
        user: UserInfo,
        token_response: AccessTokenResponse,
    ) -> Uuid {
        let email = Uuid::new_v4().to_string();
        let password = std::format!("password:{:?}", String::new_random(124));
        let first_name = user.first_name;
        let last_name = user.last_name;
        let language_code = "ru-ru".to_owned();

        let (password_sha512, password_salt) = generate_hash_and_salt_for_text(&password);

        let user_input = OwnedUser {
            user_id: user.user_id,
            alias: None,
            first_name,
            last_name,
            email,
            password_salt,
            password_sha512,
            access_token: token_response.token,
            phone_number: None,
            language_code,
            avatar: None,
            country_code: None,
        };

        user_db
            .insert_user(&user_input)
            .await
            .expect("user created")
    }

    #[tokio::test]
    async fn two_parallel_requests_receives_the_same_refreshed_tokens() {
        let pool = crate::pg_pool()
            .await
            .expect("failed to create postgres pool");
        let user_db = PgUserDb::new(pool);

        let (user, token_response) = create_token();
        let old_token = token_response.token.clone();

        let user_id = create_user(&user_db, user.clone(), token_response).await;

        let expired_token = make_expired_token(user, Duration::microseconds(1));
        let updated = user_db
            .update_user_token(&user_id, &expired_token.token, &old_token)
            .await
            .expect("token updated");
        assert!(updated > 0);

        async fn no_error_authenticate_with_token(
            token: impl AsRef<str>,
            user_db: &impl UserDb,
        ) -> (AccessTokenResponse, UserInfo) {
            authenticate_with_token(token.as_ref(), user_db)
                .await
                .expect("valid token")
        }

        let refresh_a = no_error_authenticate_with_token(&expired_token.token, &user_db);
        let refresh_b = no_error_authenticate_with_token(&expired_token.token, &user_db);
        let refresh_c = no_error_authenticate_with_token(&expired_token.token, &user_db);
        let refresh_d = no_error_authenticate_with_token(&expired_token.token, &user_db);
        let refresh_e = no_error_authenticate_with_token(&expired_token.token, &user_db);

        let (result_a, result_b, result_c, result_d, result_e) =
            tokio::join!(refresh_a, refresh_b, refresh_c, refresh_d, refresh_e);
        assert_ne!(result_a.0.token, expired_token.token);
        assert_ne!(result_b.0.token, expired_token.token);
        assert_ne!(result_c.0.token, expired_token.token);
        assert_ne!(result_d.0.token, expired_token.token);
        assert_ne!(result_e.0.token, expired_token.token);

        assert_eq!(result_a.0.token, result_b.0.token);
        assert_eq!(result_a.0.token, result_c.0.token);
        assert_eq!(result_a.0.token, result_d.0.token);
        assert_eq!(result_a.0.token, result_e.0.token);
    }
}
