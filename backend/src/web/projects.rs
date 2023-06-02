use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_auth::AuthBearer;
use crate::models::project::ProjectDb;
use crate::models::user::UserDb;
use crate::web_service::WebService;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::errors::DbError;
use crate::utils::tokens::AccessTokenResponse;
use crate::web::authentication::authenticate_with_token;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateProject {
    name: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateProjectResponseBody {
    project_id: Uuid,
    access: AccessTokenResponse,
}

#[derive(Debug)]
pub enum CreateProjectErrorResponse {
    DbError(DbError),
}

impl IntoResponse for CreateProjectErrorResponse {
    fn into_response(self) -> Response {
        match self {
            CreateProjectErrorResponse::DbError(db_error) => db_error.into_response(),
        }
    }
}

/// Registers a new user
///
/// TODO: add docs
#[tracing::instrument(skip(web_service))]
pub async fn post<UDB: UserDb, PDB: ProjectDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    AuthBearer(token): AuthBearer,
    _body_or_error: Result<Json<CreateProject>, JsonRejection>,
) -> Result<(StatusCode, Json<CreateProjectResponseBody>), CreateProjectErrorResponse> {

    let new_token = authenticate_with_token(token, &web_service.user_db).await;

    // let token = AccessToken::from_token(token).expect("TODO");
    // token.get_user();

    // 1. Check token
    // 2. Refresh token if expired
    // 3. create project
    // 4. return data

    todo!()
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn should_register_user_with_valid_parameters() {

        // let router = create_test_router().await;
        //
        // let uri = "/api/user/login";
        //
        // let login_data = LoginUserData { email, password };
        //
        // let request_body = LoginUserDataBody { data: login_data };
        //
        // post(&router, uri, &request_body).await

    }
}