use crate::models::errors::DbError;
use crate::web_service::{ErrorCode, ErrorResponseBody};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use lazy_static::lazy_static;

pub const INVALID_MAIL_MSG: &str ="your email isn`t valid";
pub const INTERNAL_SERVER_ERROR_MSG: &str = "Internal server error";
pub const SERVICE_UNAVAILABLE_MSG: &str = "Service unavailable";
const NOT_FOUND_ERROR_MSG: &str = "Not found";
pub const INVALID_TOKEN_FORMAT_ERROR_MSG: &str = "Invalid token format";
pub const UNAUTHORIZED_ERROR_MSG: &str = "Unauthorized, please try to login again";

type ErrorResponseType = (StatusCode, Json<ErrorResponseBody>);

lazy_static! {
    pub static ref NOT_FOUND_RESPONSE: ErrorResponseType = (
        StatusCode::NOT_FOUND,
        Json(ErrorResponseBody {
            code: None,
            error: NOT_FOUND_ERROR_MSG.into(),
        })
    );
    pub static ref INTERNAL_SERVER_ERROR_RESPONSE: ErrorResponseType = (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponseBody {
            code: None,
            error: INTERNAL_SERVER_ERROR_MSG.into(),
        }),
    );
    pub static ref UNAUTHORIZED_ERROR_RESPONSE: ErrorResponseType = (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponseBody {
            code: None,
            error: UNAUTHORIZED_ERROR_MSG.into(),
        }),
    );
    pub static ref STATUS_CODE_430: StatusCode =
        StatusCode::from_u16(430).unwrap_or_else(|error| {
            debug_assert!(false, "Can not create StatusCode for 430, error: {}", error);
            StatusCode::BAD_REQUEST
        });
}

pub fn create_invalid_response(error: String) -> ErrorResponseType {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponseBody {
            code: Some(ErrorCode::InvalidInput),
            error,
        }),
    )
}

pub fn create_internal_server_error(error: String) -> ErrorResponseType {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponseBody { code: None, error }),
    )
}

pub fn create_bad_request_error(error: String) -> ErrorResponseType {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponseBody { code: None, error }),
    )
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let result = match self {
            DbError::NotFoundError => NOT_FOUND_RESPONSE.to_owned(),
            DbError::UnavailableTryAgain => (
                STATUS_CODE_430.to_owned(),
                Json(ErrorResponseBody {
                    code: None,
                    error: SERVICE_UNAVAILABLE_MSG.into(),
                }),
            ),
            DbError::UnexpectedError(error) => {
                tracing::error!(error);
                INTERNAL_SERVER_ERROR_RESPONSE.to_owned()
            }
        };
        result.into_response()
    }
}
