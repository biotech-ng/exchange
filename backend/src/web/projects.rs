use crate::models::errors::DbError;
use crate::models::project::ProjectDb;
use crate::models::user::UserDb;
use crate::utils::tokens::AccessToken;
use crate::web::errors::{create_invalid_response, UNAUTHORIZED_ERROR_RESPONSE};
use crate::web::formats::JsonDateTime;
use crate::web_service::WebService;
use axum::extract::rejection::{JsonRejection, PathRejection};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_auth::AuthBearer;
use database::projects::{Project, ProjectInput};
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
}

#[derive(Debug)]
pub enum CreateProjectErrorResponse {
    UnAuthorized,
    DbError(DbError),
    InvalidInputDataFormat(String),
}

impl IntoResponse for CreateProjectErrorResponse {
    fn into_response(self) -> Response {
        match self {
            CreateProjectErrorResponse::UnAuthorized => {
                UNAUTHORIZED_ERROR_RESPONSE.clone().into_response()
            }
            CreateProjectErrorResponse::DbError(db_error) => db_error.into_response(),
            CreateProjectErrorResponse::InvalidInputDataFormat(error) => {
                create_invalid_response(error).into_response()
            }
        }
    }
}

/// Creates a new project
///
#[tracing::instrument(skip(web_service))]
pub async fn post<UDB: UserDb, PDB: ProjectDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    AuthBearer(token): AuthBearer,
    body_or_error: Result<Json<CreateProject>, JsonRejection>,
) -> Result<(StatusCode, Json<CreateProjectResponseBody>), CreateProjectErrorResponse> {
    let access_token =
        AccessToken::from_token(token).map_err(|_| CreateProjectErrorResponse::UnAuthorized)?;
    let user_info = access_token.get_user();

    let request = body_or_error
        .map_err(|x| x.to_string())
        .map_err(CreateProjectErrorResponse::InvalidInputDataFormat)?;

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
        Json(CreateProjectResponseBody { project_id }),
    ))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectResponseData {
    name: String,
    description: String,
    user_id: Uuid,
    created_at: JsonDateTime,
    updated_at: JsonDateTime,
}

impl From<Project> for ProjectResponseData {
    fn from(value: Project) -> Self {
        ProjectResponseData {
            name: value.name,
            description: value.description,
            user_id: value.user_id,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

#[derive(Debug)]
pub enum GetProjectErrorResponse {
    DbError(DbError),
    InvalidInputDataFormat(String),
}

impl IntoResponse for GetProjectErrorResponse {
    fn into_response(self) -> Response {
        match self {
            GetProjectErrorResponse::DbError(db_error) => db_error.into_response(),
            GetProjectErrorResponse::InvalidInputDataFormat(error) => {
                create_invalid_response(error).into_response()
            }
        }
    }
}

/// Creates a new doc
///
#[tracing::instrument(skip(web_service))]
pub async fn get<UDB: UserDb, PDB: ProjectDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    project_id_or_error: Result<Path<Uuid>, PathRejection>,
) -> Result<(StatusCode, Json<ProjectResponseData>), GetProjectErrorResponse> {
    let project_id = project_id_or_error
        .map_err(|x| x.to_string())
        .map_err(GetProjectErrorResponse::InvalidInputDataFormat)?;

    let project = web_service
        .project_db
        .get_project_by_id(&project_id)
        .await
        .map_err(GetProjectErrorResponse::DbError)?;

    Ok((StatusCode::CREATED, Json(project.into())))
}

#[cfg(test)]
mod tests {
    use crate::web::projects::{CreateProject, CreateProjectResponseBody, ProjectResponseData};
    use crate::web::users::tests::{
        create_test_router, get_auth_header_for_name, register_new_user,
    };
    use crate::web_service::tests::{
        deserialize_response_body, get, get_with_auth_header, post_with_auth_header,
    };
    use database::utils::random_samples::RandomSample;
    use uuid::Uuid;

    async fn create_project() -> (CreateProject, CreateProjectResponseBody, String) {
        let (_, response) = register_new_user(None).await;

        let auth_token = get_auth_header_for_name(&response);

        let router = create_test_router().await;

        let name = String::new_random(100);
        let description = String::new_random(100);

        let request_body = CreateProject {
            name: name.clone(),
            description: description.clone(),
        };

        let uri = "/api/project/new";

        let response = post_with_auth_header(&router, uri, &request_body, Some(&auth_token)).await;
        assert_eq!(response.status(), 201);

        let auth_tokens = get_auth_header_for_name(&response);
        assert_eq!(auth_tokens, auth_token);

        let create_project_response =
            deserialize_response_body::<CreateProjectResponseBody>(response).await;

        (request_body, create_project_response, auth_token)
    }

    #[tokio::test]
    async fn should_create_a_new_project_for_a_given_user() {
        let router = create_test_router().await;

        let (create_project_request, create_project_response, token) = create_project().await;

        let uri = std::format!("/api/project/{}", create_project_response.project_id);
        let response = get_with_auth_header(&router, uri, Some(&token)).await;
        assert_eq!(response.status(), 201);

        let auth_tokens = get_auth_header_for_name(&response);
        assert_eq!(auth_tokens, token);

        let project_response = deserialize_response_body::<ProjectResponseData>(response).await;
        assert_eq!(project_response.name, create_project_request.name);
        assert_eq!(
            project_response.description,
            create_project_request.description
        );
    }

    #[tokio::test]
    async fn should_return_unauthorized_error_when_no_auth_token_is_provided() {
        let router = create_test_router().await;

        let uri = std::format!("/api/project/{}", Uuid::new_v4());
        let response = get(&router, uri).await;
        assert_eq!(response.status(), 401);
    }
}
