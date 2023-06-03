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
        user_id: user_info.user_id,
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
    use database::utils::random_samples::RandomSample;
    use crate::web::projects::{CreateProject, CreateProjectResponseBody};
    use crate::web::users::RegisterUserResponseBody;
    use crate::web::users::tests::{create_test_router, register_new_user};
    use crate::web_service::tests::{deserialize_response_body, post_with_auth_header};

    #[tokio::test]
    async fn should_create_a_new_project_for_a_given_user() {
        let (_, response) = register_new_user(None).await;

        let token = deserialize_response_body::<RegisterUserResponseBody>(response).await.into_token();

        let router = create_test_router().await;

        let name = String::new_random(100);
        let description = String::new_random(100);

        let request_body = CreateProject {
            name,
            description,
        };

        let uri = "/api/project/new";

        let response = post_with_auth_header(&router, uri, &request_body, Some(&token)).await;
        assert_eq!(response.status(), 201);

        let create_project_response = deserialize_response_body::<CreateProjectResponseBody>(response).await;
        assert_eq!(create_project_response.access.token, token);
    }
}
