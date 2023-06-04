use crate::errors::errors::DbError;
use crate::models::project::ProjectDb;
use crate::models::user::UserDb;
use crate::utils::tokens::AccessTokenResponse;
use crate::web::authentication::authenticate_with_token;
use crate::web_service::WebService;
use axum::extract::rejection::{JsonRejection, PathRejection};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_auth::AuthBearer;
use database::projects::{Project, ProjectInput};
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
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
    DbError(DbError),
}

impl IntoResponse for CreateProjectErrorResponse {
    fn into_response(self) -> Response {
        match self {
            CreateProjectErrorResponse::DbError(db_error) => db_error.into_response(),
        }
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
        authenticate_with_token(token, &web_service.user_db).await.expect("TODO");
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
        .map_err(CreateProjectErrorResponse::DbError)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateProjectResponseBody {
            project_id,
            access: access_token_response,
        }),
    ))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectResponseData {
    name: String,
    description: String,
    user_id: Uuid,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
}

impl From<Project> for ProjectResponseData {
    fn from(value: Project) -> Self {
        ProjectResponseData {
            name: value.name,
            description: value.description,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectResponseBody {
    project: ProjectResponseData,
    access: AccessTokenResponse,
}

/// Creates a new doc
///
/// TODO: add docs
/// TODO: Change error type
#[tracing::instrument(skip(web_service))]
pub async fn get<UDB: UserDb, PDB: ProjectDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    AuthBearer(token): AuthBearer,
    project_id_or_error: Result<Path<Uuid>, PathRejection>,
) -> Result<(StatusCode, Json<ProjectResponseBody>), CreateProjectErrorResponse> {
    let (access_token_response, _) = authenticate_with_token(token, &web_service.user_db).await.expect("TODO");
    let project_id = project_id_or_error.expect("TODO");

    let project = web_service
        .project_db
        .get_project_by_id(&project_id)
        .await
        .expect("TODO");

    Ok((
        StatusCode::CREATED,
        Json(ProjectResponseBody {
            project: project.into(),
            access: access_token_response,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use crate::web::projects::{CreateProject, CreateProjectResponseBody, ProjectResponseBody};
    use crate::web::users::tests::{create_test_router, register_new_user};
    use crate::web::users::RegisterUserResponseBody;
    use crate::web_service::tests::{
        deserialize_response_body, get_with_auth_header, post_with_auth_header,
    };
    use database::utils::random_samples::RandomSample;

    async fn create_project() -> (CreateProject, CreateProjectResponseBody, String) {
        let (_, response) = register_new_user(None).await;

        let token = deserialize_response_body::<RegisterUserResponseBody>(response)
            .await
            .into_token();

        let router = create_test_router().await;

        let name = String::new_random(100);
        let description = String::new_random(100);

        let request_body = CreateProject {
            name: name.clone(),
            description: description.clone(),
        };

        let uri = "/api/project/new";

        let response = post_with_auth_header(&router, uri, &request_body, Some(&token)).await;
        assert_eq!(response.status(), 201);

        let create_project_response =
            deserialize_response_body::<CreateProjectResponseBody>(response).await;
        assert_eq!(create_project_response.access.token, token);

        (request_body, create_project_response, token)
    }

    #[tokio::test]
    async fn should_create_a_new_project_for_a_given_user() {
        let router = create_test_router().await;

        let (create_project_request, create_project_response, token) = create_project().await;

        let uri = std::format!("/api/project/{}", create_project_response.project_id);
        let response = get_with_auth_header(&router, uri, Some(&token)).await;
        assert_eq!(response.status(), 201);

        let project_response = deserialize_response_body::<ProjectResponseBody>(response).await;
        assert_eq!(project_response.access.token, token);
        assert_eq!(project_response.project.name, create_project_request.name);
        assert_eq!(
            project_response.project.description,
            create_project_request.description
        );
    }
}
