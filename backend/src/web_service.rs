use crate::models::project::ProjectDb;
use crate::models::user::UserDb;
use crate::web::authentication::check_and_refresh_auth_token;
use crate::web::{projects, users};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get;
use axum::{middleware, routing::post, Router};
use axum_tracing_opentelemetry::{find_current_trace_id, opentelemetry_tracing_layer};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    AlreadyRegistered,
    InvalidInput,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ErrorResponseBody {
    pub code: Option<ErrorCode>,
    pub error: String,
}

#[derive(Clone)]
pub struct WebService<UDB, PDB> {
    pub user_db: UDB,
    pub project_db: PDB,
}

impl<UDB: UserDb, PDB: ProjectDb> WebService<UDB, PDB> {
    pub fn new(user_db: UDB, project_db: PDB) -> Self {
        Self {
            user_db,
            project_db,
        }
    }

    pub fn into_router(self) -> Router {
        Router::new()
            .route("/api/project/new", post(projects::post))
            .route("/api/project/:payment_id", get(projects::get))
            .layer(middleware::from_fn_with_state(
                self.clone(),
                check_and_refresh_auth_token,
            ))
            .route("/api/user", post(users::post))
            .route("/api/user/login", post(users::login))
            .layer(middleware::from_fn(propagate_b3_headers))
            .layer(opentelemetry_tracing_layer())
            .with_state(self)
    }
}

async fn propagate_b3_headers<B>(req: Request<B>, next: Next<B>) -> Result<Response, Response> {
    let mut response = next.run(req).await;

    match find_current_trace_id() {
        Some(trace_id) => match trace_id.parse() {
            Ok(value) => {
                response.headers_mut().insert("x-b3-trace", value);
            }
            Err(error) => tracing::error!("trace_id: {trace_id} parsing error: {error}"),
        },
        None => {
            tracing::warn!("could not find a trace id");
        }
    }

    Ok(response)
}

#[cfg(test)]
pub mod tests {
    use crate::models::project::PgProjectDb;
    use crate::models::user::PgUserDb;
    use crate::utils::modify_builder::ModifyBuilder;
    use axum::http::request::Builder;
    use axum::{
        body::Bytes,
        http::{header::CONTENT_TYPE, Method, Request},
    };
    use http_body::combinators::UnsyncBoxBody;
    use serde::{de::DeserializeOwned, Serialize};
    use std::fmt::Display;
    use tower::ServiceExt;

    use super::*;

    impl WebService<PgUserDb, PgProjectDb> {
        pub async fn new_test() -> Self {
            let pool = crate::pg_pool()
                .await
                .expect("failed to create postgres pool");
            Self {
                user_db: PgUserDb::new(pool.clone()),
                project_db: PgProjectDb::new(pool),
            }
        }
    }

    pub async fn send_request(
        router: &Router,
        request: Request<hyper::Body>,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        router
            .clone()
            .oneshot(request)
            .await
            .expect("failed to send oneshot request")
    }

    impl ModifyBuilder for Builder {}

    pub async fn get_with_auth_header(
        router: &Router,
        uri: impl AsRef<str>,
        token: Option<impl AsRef<str> + Display>,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        let request = Request::builder()
            .method(Method::GET)
            .uri(uri.as_ref())
            .modify(token.as_ref(), |this, token| {
                this.header("Authorization", std::format!("Bearer {token}"))
            })
            .body(hyper::Body::empty())
            .expect("failed to build GET request");
        send_request(router, request).await
    }

    pub async fn get(
        router: &Router,
        uri: impl AsRef<str>,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        get_with_auth_header(router, uri, Option::<&str>::None).await
    }

    pub async fn post_with_auth_header(
        router: &Router,
        uri: impl AsRef<str>,
        body: &impl Serialize,
        token: Option<impl AsRef<str> + Display>,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        let request = Request::builder()
            .method(Method::POST)
            .uri(uri.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .modify(token.as_ref(), |this, token| {
                this.header("Authorization", std::format!("Bearer {token}"))
            })
            .body(
                serde_json::to_vec(body)
                    .expect("failed to serialize POST body")
                    .into(),
            )
            .expect("failed to build POST request");
        send_request(router, request).await
    }

    pub async fn post(
        router: &Router,
        uri: impl AsRef<str>,
        body: &impl Serialize,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        post_with_auth_header(router, uri, body, Option::<String>::None).await
    }

    pub async fn deserialize_response_body<T>(
        response: hyper::Response<UnsyncBoxBody<Bytes, axum::Error>>,
    ) -> T
    where
        T: DeserializeOwned,
    {
        let bytes = hyper::body::to_bytes(response.into_body())
            .await
            .expect("failed to read response body into bytes");
        serde_json::from_slice::<T>(&bytes).expect("failed to deserialize response")
    }
}
