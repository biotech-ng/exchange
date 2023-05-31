use crate::errors::errors::DbError;
use crate::models::project::projects::ProjectDb;
use crate::models::user::users::{OwnedUser, UserDb};
use crate::utils::salted_hashes::{
    generate_b64_hash_for_text_and_salt, generate_hash_and_salt_for_text,
};
use crate::utils::tokens::{TokenResponse, UserInfo};
use crate::web_service::{ErrorCode, ErrorResponseBody, WebService};
use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode, Json};
use database::users::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterUserData {
    email: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
    language_code: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterUserRequestBody {
    data: RegisterUserData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterUserResponseBody {
    data: RegisterUserData,
    token: TokenResponse,
}

#[derive(Debug)]
pub enum RegisterUserErrorResponse {
    DbError(DbError),
    AlreadyRegistered,
}

impl IntoResponse for RegisterUserErrorResponse {
    fn into_response(self) -> Response {
        match self {
            RegisterUserErrorResponse::DbError(db_error) => db_error.into_response(),
            RegisterUserErrorResponse::AlreadyRegistered => (
                StatusCode::ALREADY_REPORTED,
                Json(ErrorResponseBody {
                    code: Some(ErrorCode::AlreadyRegistered),
                    error: "user for given email already exists".to_owned(),
                }),
            )
                .into_response(),
        }
    }
}

fn login_user<T: AsRef<str>>(password: T, user: &User) -> Option<TokenResponse> {
    let input_hash =
        generate_b64_hash_for_text_and_salt(password, &user.password_salt).expect("TODO");
    let existing_hash = &user.password_sha512;
    if existing_hash != &input_hash {
        return None;
    }

    Some(
        TokenResponse::new(UserInfo {
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
        })
        .expect("TODO"),
    )
}

/// Registers a new user
///
/// TODO: add docs
// 1. Validate input, each field, format and length
// 2. create salt
// 3. encode password
// 4. prepare tokens response
#[tracing::instrument(skip(web_service))]
pub async fn post<UDB: UserDb, PDB: ProjectDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    body_or_error: Result<Json<RegisterUserRequestBody>, JsonRejection>,
) -> Result<(StatusCode, Json<RegisterUserResponseBody>), RegisterUserErrorResponse> {
    let Json(body) = body_or_error.unwrap(); // TODO validate response

    // TODO: don't fetch whole user
    let user_or_error = web_service
        .user_db
        .get_user_by_email(&body.data.email)
        .await;
    match user_or_error {
        Ok(user) => {
            if let Some(token_response) = login_user(&body.data.password, &user) {
                web_service
                    .user_db
                    .update_user_token(&user.id, &token_response.token)
                    .await
                    .map_err(RegisterUserErrorResponse::DbError)?;

                Ok((
                    StatusCode::ACCEPTED,
                    Json(RegisterUserResponseBody {
                        data: body.data,
                        token: token_response,
                    }),
                ))
            } else {
                Err(RegisterUserErrorResponse::AlreadyRegistered)
            }
        }
        Err(DbError::NotFoundError) => {
            let (password_sha512, password_salt) =
                generate_hash_and_salt_for_text(&body.data.password);

            let token_response = TokenResponse::new(UserInfo {
                first_name: body.data.first_name.clone(),
                last_name: body.data.last_name.clone(),
            })
            .expect("TODO");

            let user = OwnedUser {
                alias: None,
                first_name: body.data.first_name.clone(),
                last_name: body.data.last_name.clone(),
                email: body.data.email.clone(),
                password_salt,
                password_sha512,
                access_token: token_response.token.clone(),
                phone_number: None,
                language_code: body.data.language_code.clone(),
                avatar: None,
                country_code: None,
            };

            let _ = web_service
                .user_db
                .insert_user(&user)
                .await
                .map_err(RegisterUserErrorResponse::DbError)?;

            Ok((
                StatusCode::CREATED,
                Json(RegisterUserResponseBody {
                    data: body.data,
                    token: token_response,
                }),
            ))
        }
        Err(db_error) => Err(RegisterUserErrorResponse::DbError(db_error)),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginUserData {
    email: String,
    password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginUserDataBody {
    data: LoginUserData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginUserResponseBody {
    token: TokenResponse,
}

#[derive(Debug)]
pub enum LoginUserErrorResponse {
    DbError(DbError),
    NotFound,
    InvalidPassword,
}

impl IntoResponse for LoginUserErrorResponse {
    fn into_response(self) -> Response {
        match self {
            LoginUserErrorResponse::DbError(db_error) => db_error.into_response(),
            LoginUserErrorResponse::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponseBody {
                    code: None,
                    error: "user for a given email is not registered".to_owned(),
                }),
            )
                .into_response(),
            LoginUserErrorResponse::InvalidPassword => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponseBody {
                    code: None,
                    error: "Entered password is wrong, please try again".to_owned(),
                }),
            )
                .into_response(),
        }
    }
}

/// Login existing user
///
/// TODO: add docs
#[tracing::instrument(skip(web_service))]
pub async fn login<UDB: UserDb, PDB: ProjectDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    body_or_error: Result<Json<LoginUserDataBody>, JsonRejection>,
) -> Result<(StatusCode, Json<LoginUserResponseBody>), LoginUserErrorResponse> {
    let Json(body) = body_or_error.unwrap(); // TODO validate response

    let user_or_error = web_service
        .user_db
        .get_user_by_email(&body.data.email)
        .await;
    match user_or_error {
        Ok(user) => {
            if let Some(token_response) = login_user(&body.data.password, &user) {
                web_service
                    .user_db
                    .update_user_token(&user.id, &token_response.token)
                    .await
                    .map_err(LoginUserErrorResponse::DbError)?;

                Ok((
                    StatusCode::ACCEPTED,
                    Json(LoginUserResponseBody {
                        token: token_response,
                    }),
                ))
            } else {
                Err(LoginUserErrorResponse::InvalidPassword)
            }
        }
        Err(DbError::NotFoundError) => Err(LoginUserErrorResponse::NotFound),
        Err(db_error) => Err(LoginUserErrorResponse::DbError(db_error)),
    }
}

#[cfg(test)]
mod tests {
    use crate::models::user::users::{PgUserDb, UserDb};
    use crate::utils::salted_hashes::{
        generate_b64_hash_for_text_and_salt, generate_hash_and_salt_for_text,
    };
    use crate::utils::tokens::AccessToken;
    use crate::web::users::{
        LoginUserData, LoginUserDataBody, LoginUserResponseBody, RegisterUserData,
        RegisterUserRequestBody, RegisterUserResponseBody,
    };
    use crate::web_service::tests::{deserialize_response_body, post};
    use crate::web_service::{ErrorCode, ErrorResponseBody, WebService};
    use axum::body::Bytes;
    use axum::Router;
    use database::utils::random_samples::RandomSample;
    use http_body::combinators::UnsyncBoxBody;
    use uuid::Uuid;

    async fn create_test_router() -> Router {
        WebService::new_test().await.into_router()
    }

    #[test]
    fn test_passwords() {
        let password = String::new_random(1024);

        let (hash, salt) = generate_hash_and_salt_for_text(&password);

        let hash2 = generate_b64_hash_for_text_and_salt(password, salt).expect("valid salt");

        assert_eq!(hash, hash2)
    }

    async fn register_new_user(
        user_data: Option<RegisterUserData>,
    ) -> (
        RegisterUserData,
        hyper::Response<UnsyncBoxBody<Bytes, axum::Error>>,
    ) {
        let router = create_test_router().await;

        let email = Uuid::new_v4().to_string();
        let password = std::format!("password:{:?}", Uuid::new_v4().to_string());
        let first_name = std::format!("fn:{:?}", Uuid::new_v4().to_string());
        let last_name = std::format!("ln:{:?}", Uuid::new_v4().to_string());
        let language_code = "ru-ru".to_owned();

        let uri = "/api/user";

        let result = user_data.unwrap_or(RegisterUserData {
            email: email.clone(),
            password,
            first_name: Some(first_name.clone()),
            last_name: Some(last_name.clone()),
            language_code,
        });

        let request_body = RegisterUserRequestBody {
            data: result.clone(),
        };

        // Registration ok
        let response = post(&router, uri, &request_body).await;

        (result, response)
    }

    #[tokio::test]
    async fn should_register_user_with_valid_parameters() {
        let (request, response) = register_new_user(None).await;

        assert_eq!(response.status(), 201);

        let response_body = deserialize_response_body::<RegisterUserResponseBody>(response).await;
        assert_eq!(response_body.data.first_name, request.first_name);
        assert_eq!(response_body.data.last_name, request.last_name);

        // Token ok
        let parsed_access_token =
            AccessToken::from_token(response_body.token.token).expect("valid token");

        assert_eq!(
            parsed_access_token.get_user().first_name,
            request.first_name
        );
        assert_eq!(parsed_access_token.get_user().last_name, request.last_name);

        // Test created user
        let pool = crate::pg_pool()
            .await
            .expect("failed to create postgres pool");
        let user_db = PgUserDb::new(pool);

        let user = user_db
            .get_user_by_email(&request.email)
            .await
            .expect("user exists");

        assert_eq!(user.email, request.email);
        assert_eq!(user.first_name, request.first_name);
        assert_eq!(user.last_name, request.last_name);
    }

    #[tokio::test]
    async fn should_reject_registration_with_same_email_but_different_password() {
        let (user_fields, _) = register_new_user(None).await;

        let password = std::format!("password:{:?}", Uuid::new_v4().to_string());

        let user_fields = RegisterUserData {
            password,
            ..user_fields
        };

        let (_, response) = register_new_user(Some(user_fields)).await;

        assert_eq!(response.status(), 208);

        let error_response_body = deserialize_response_body::<ErrorResponseBody>(response).await;

        assert_eq!(
            error_response_body.error,
            "user for given email already exists"
        );
        assert_eq!(error_response_body.code, Some(ErrorCode::AlreadyRegistered));
    }

    #[tokio::test]
    async fn should_login_when_registering_with_same_email_and_password() {
        let (request, _) = register_new_user(None).await;

        let (_, response) = register_new_user(Some(request.clone())).await;

        assert_eq!(response.status(), 202);

        let response_body = deserialize_response_body::<RegisterUserResponseBody>(response).await;
        assert_eq!(response_body.data.first_name, request.first_name);
        assert_eq!(response_body.data.last_name, request.last_name);

        // Token ok
        let parsed_access_token =
            AccessToken::from_token(response_body.token.token).expect("valid token");

        assert_eq!(
            parsed_access_token.get_user().first_name,
            request.first_name
        );
        assert_eq!(parsed_access_token.get_user().last_name, request.last_name);

        // Test created user
        let pool = crate::pg_pool()
            .await
            .expect("failed to create postgres pool");
        let user_db = PgUserDb::new(pool);

        let user = user_db
            .get_user_by_email(&request.email)
            .await
            .expect("user exists");

        assert_eq!(user.email, request.email);
        assert_eq!(user.first_name, request.first_name);
        assert_eq!(user.last_name, request.last_name);
    }

    async fn login_with_email_and_password(
        email: String,
        password: String,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        let router = create_test_router().await;

        let uri = "/api/user/login";

        let login_data = LoginUserData { email, password };

        let request_body = LoginUserDataBody { data: login_data };

        post(&router, uri, &request_body).await
    }

    #[tokio::test]
    async fn login_should_fail_for_non_existing_email() {
        let email = Uuid::new_v4().to_string();
        let password = std::format!("password:{:?}", Uuid::new_v4().to_string());

        let response = login_with_email_and_password(email, password).await;

        assert_eq!(response.status(), 404);

        let response_body = deserialize_response_body::<ErrorResponseBody>(response).await;

        assert_eq!(
            response_body.error,
            "user for a given email is not registered"
        );
    }

    #[tokio::test]
    async fn login_should_fail_for_a_wrong_password() {
        let (request, response) = register_new_user(None).await;

        assert_eq!(response.status(), 201);

        let email = request.email;
        let password = std::format!("password:{:?}", Uuid::new_v4().to_string());

        let response = login_with_email_and_password(email, password).await;

        assert_eq!(response.status(), 401);

        let response_body = deserialize_response_body::<ErrorResponseBody>(response).await;

        assert_eq!(
            response_body.error,
            "Entered password is wrong, please try again"
        );
    }

    #[tokio::test]
    async fn should_successfully_login_with_a_correct_password() {
        let (request, response) = register_new_user(None).await;

        assert_eq!(response.status(), 201);

        let email = request.email.clone();
        let password = request.password;

        let response = login_with_email_and_password(email, password).await;

        assert_eq!(response.status(), 202);

        let response_body = deserialize_response_body::<LoginUserResponseBody>(response).await;

        // Test token
        let pool = crate::pg_pool()
            .await
            .expect("failed to create postgres pool");
        let user_db = PgUserDb::new(pool);

        let user = user_db
            .get_user_by_email(&request.email)
            .await
            .expect("user exists");

        assert_eq!(response_body.token.token, user.access_token)
    }
}
