use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::models::project::ProjectDb;
use crate::models::user::UserDb;
use crate::web_service::WebService;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateProject {
    email: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
    language_code: String,
}

/// Registers a new user
///
/// TODO: add docs
// #[tracing::instrument(skip(web_service))]
// pub async fn post<UDB: UserDb, PDB: ProjectDb>(
//     State(web_service): State<WebService<UDB, PDB>>,
//     body_or_error: Result<Json<RegisterUserRequestBody>, JsonRejection>,
// ) -> Result<(StatusCode, Json<RegisterUserResponseBody>), RegisterUserErrorResponse> {
//     let Json(body) = body_or_error.unwrap(); // TODO validate response
//
//     // TODO: don't fetch whole user
//     let user_or_error = web_service
//         .user_db
//         .get_user_by_email(&body.data.email)
//         .await;
//     match user_or_error {
//         Ok(user) => {
//             if let Some(token_response) = login_user(&body.data.password, &user) {
//                 web_service
//                     .user_db
//                     .update_user_token(&user.id, &token_response.token)
//                     .await
//                     .map_err(RegisterUserErrorResponse::DbError)?;
//
//                 Ok((
//                     StatusCode::ACCEPTED,
//                     Json(RegisterUserResponseBody {
//                         data: body.data,
//                         token: token_response,
//                     }),
//                 ))
//             } else {
//                 Err(RegisterUserErrorResponse::AlreadyRegistered)
//             }
//         }
//         Err(DbError::NotFoundError) => {
//             let (password_sha512, password_salt) =
//                 generate_hash_and_salt_for_text(&body.data.password);
//
//             let token_response = TokenResponse::new(UserInfo {
//                 first_name: body.data.first_name.clone(),
//                 last_name: body.data.last_name.clone(),
//             })
//                 .expect("TODO");
//
//             let user = OwnedUser {
//                 alias: None,
//                 first_name: body.data.first_name.clone(),
//                 last_name: body.data.last_name.clone(),
//                 email: body.data.email.clone(),
//                 password_salt,
//                 password_sha512,
//                 access_token: token_response.token.clone(),
//                 phone_number: None,
//                 language_code: body.data.language_code.clone(),
//                 avatar: None,
//                 country_code: None,
//             };
//
//             let _ = web_service
//                 .user_db
//                 .insert_user(&user)
//                 .await
//                 .map_err(RegisterUserErrorResponse::DbError)?;
//
//             Ok((
//                 StatusCode::CREATED,
//                 Json(RegisterUserResponseBody {
//                     data: body.data,
//                     token: token_response,
//                 }),
//             ))
//         }
//         Err(db_error) => Err(RegisterUserErrorResponse::DbError(db_error)),
//     }
// }
