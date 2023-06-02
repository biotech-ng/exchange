use crate::models::project::ProjectDb;
use crate::models::user::UserDb;
use crate::utils::tokens::AccessTokenResponse;
use crate::web::authentication::authenticate_with_token;
use crate::web_service::WebService;
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_auth::AuthBearer;
use database::projects::ProjectInput;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    // DbError(DbError),
}

impl IntoResponse for CreateProjectErrorResponse {
    fn into_response(self) -> Response {
        // match self {
        //     CreateProjectErrorResponse::DbError(db_error) => db_error.into_response(),
        // }
        todo!()
    }
}

/// Creates a new doc
///
/// TODO: add docs
#[tracing::instrument(skip(web_service))]
pub async fn post<UDB: UserDb, PDB: ProjectDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    AuthBearer(token): AuthBearer,
    body_or_error: Result<Json<CreateProject>, JsonRejection>,
) -> Result<(StatusCode, Json<CreateProjectResponseBody>), CreateProjectErrorResponse> {
    let (access_token_response, user_info) =
        authenticate_with_token(token, &web_service.user_db).await;
    let request = body_or_error.expect("TODO");

    let user_input = ProjectInput {
        name: request.name.clone(),
        description: request.description.clone(),
        user_id: user_info.user_id.clone(),
    };

    let project_id = web_service
        .project_db
        .insert_project(&user_input)
        .await
        .expect("TODO");

    Ok((
        StatusCode::CREATED,
        Json(CreateProjectResponseBody {
            project_id,
            access: access_token_response,
        }),
    ))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn should_create_a_new_project_for_a_given_user() {

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
