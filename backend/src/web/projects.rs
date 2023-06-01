use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::models::project::ProjectDb;
use crate::models::user::UserDb;
use crate::web_service::WebService;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::errors::DbError;
use crate::utils::tokens::TokenResponse;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateProject {
    name: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateProjectResponseBody {
    project_id: Uuid,
    access: TokenResponse,
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
    body_or_error: Result<Json<CreateProject>, JsonRejection>,
) -> Result<(StatusCode, Json<CreateProjectResponseBody>), CreateProjectErrorResponse> {
    todo!()
}
