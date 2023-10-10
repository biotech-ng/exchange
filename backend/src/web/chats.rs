use crate::models::chats::ChatDb;
use crate::models::errors::DbError;
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
use database::chats::{Chat, ChatType, CreateChat};
use serde::{Deserialize, Serialize};
use std::fs::canonicalize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateChatResponseBody {
    chat_id: Uuid,
}

#[derive(Debug)]
pub enum CreateChatErrorResponse {
    UnAuthorized,
    DbError(DbError),
    InvalidInputDataFormat(String),
}

impl IntoResponse for CreateChatErrorResponse {
    fn into_response(self) -> Response {
        match self {
            CreateChatErrorResponse::UnAuthorized => {
                UNAUTHORIZED_ERROR_RESPONSE.clone().into_response()
            }
            CreateChatErrorResponse::DbError(db_error) => db_error.into_response(),
            CreateChatErrorResponse::InvalidInputDataFormat(error) => {
                create_invalid_response(error).into_response()
            }
        }
    }
}

/// Creates a new Chat
///
#[tracing::instrument(skip(web_service))]
pub async fn post<UDB: UserDb, PDB: ChatDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    AuthBearer(token): AuthBearer,
    body_or_error: Result<Json<CreateChat>, JsonRejection>,
) -> Result<(StatusCode, Json<CreateChatResponseBody>), CreateChatErrorResponse> {
    let access_token =
        AccessToken::from_token(token).map_err(|_| CreateChatErrorResponse::UnAuthorized)?;
    let user_info = access_token.get_user();

    let request = body_or_error
        .map_err(|x| x.to_string())
        .map_err(CreateChatErrorResponse::InvalidInputDataFormat)?;

    let chat_input = CreateChat {
        r#type: request.r#type.clone(),
        title: request.title.clone(),
        description: request.description.clone(),
        avatar: request.avatar.clone(),
    };

    let chat_id = web_service
        .chat_db
        .insert_chat(&chat_input)
        .await
        .map_err(CreateChatErrorResponse::DbError)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateChatResponseBody { chat_id }),
    ))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponseData {
    id: Uuid,
    r#type: ChatType,
    title: String,
    description: Option<String>,
    avatar: Option<String>,
    created_at: JsonDateTime,
    updated_at: JsonDateTime,
}

impl From<Chat> for ChatResponseData {
    fn from(value: Chat) -> Self {
        ChatResponseData {
            id: value.id,
            r#type: value.r#type,
            title: value.title,
            description: value.description,
            avatar: value.avatar,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

#[derive(Debug)]
pub enum GetChatErrorResponse {
    DbError(DbError),
    InvalidInputDataFormat(String),
}

impl IntoResponse for GetChatErrorResponse {
    fn into_response(self) -> Response {
        match self {
            GetChatErrorResponse::DbError(db_error) => db_error.into_response(),
            GetChatErrorResponse::InvalidInputDataFormat(error) => {
                create_invalid_response(error).into_response()
            }
        }
    }
}

/// Creates a new doc
///
#[tracing::instrument(skip(web_service))]
pub async fn get<UDB: UserDb, PDB: ChatDb>(
    State(web_service): State<WebService<UDB, PDB>>,
    chat_id_or_error: Result<Path<Uuid>, PathRejection>,
) -> Result<(StatusCode, Json<ChatResponseData>), GetChatErrorResponse> {
    let chat_id = chat_id_or_error
        .map_err(|x| x.to_string())
        .map_err(GetChatErrorResponse::InvalidInputDataFormat)?;

    let chat_info = web_service
        .chat_db
        .get_chat(&chat_id)
        .await
        .map_err(GetChatErrorResponse::DbError)?;

    Ok((StatusCode::CREATED, Json(chat_info.into())))
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
