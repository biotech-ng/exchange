use crate::web::users;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{middleware, routing::post, Router};
use axum_tracing_opentelemetry::{find_current_trace_id, opentelemetry_tracing_layer};
use serde::{Deserialize, Serialize};
use crate::models::user::users::UserDb;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    AlreadyRegistered,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ErrorResponseBody {
    pub code: Option<ErrorCode>,
    pub error: String,
}

#[derive(Clone)]
pub struct WebService<UDB> {
    pub user_db: UDB,
}

impl<UDB: UserDb> WebService<UDB> {
    pub fn new(user_db: UDB) -> Self {
        Self { user_db }
    }

    pub fn into_router(self) -> Router {
        Router::new()
            .route("/api/user", post(users::post))
            .route("/api/user/login", post(users::login))
            // .route("/api/payments", post(payments::post/*::<T, PDB, RDB>*/))
            // .route(
            //     "/api/payments/:payment_id",
            //     get(payments::get/*::<T, PDB, RDB>*/),
            // )
            // .route(
            //     "/api/payments/:payment_id/refunds",
            //     post(refunds::post/*::<T, PDB, RDB>*/),
            // )
            // .route(
            //     "/api/payments/:payment_id/refunds/:refund_id",
            //     get(refunds::get/*::<T, PDB, RDB>*/),
            // )
            .layer(middleware::from_fn(propagate_b3_headers))
            .layer(opentelemetry_tracing_layer())
            .with_state(self)
            .with_state(())
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
    use async_recursion::async_recursion;
    use axum::{
        body::Bytes,
        http::{header::CONTENT_TYPE, Method, Request},
    };
    use http_body::combinators::UnsyncBoxBody;
    use serde::{de::DeserializeOwned, Serialize};
    use tower::ServiceExt;
    use crate::models::user::users::PgUserDb;

    use super::*;

    impl WebService<PgUserDb> {
        pub async fn new_test() -> Self {
            let pool = crate::pg_pool()
                .await
                .expect("failed to create postgres pool");
            Self {
                user_db: PgUserDb::new(pool),
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

    // pub async fn get(
    //     router: &Router,
    //     uri: impl AsRef<str>,
    // ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
    //     let request = Request::builder()
    //         .method(Method::GET)
    //         .uri(uri.as_ref())
    //         .body(hyper::Body::empty())
    //         .expect("failed to build GET request");
    //     send_request(router, request).await
    // }

    #[async_recursion(?Send)]
    pub async fn post<T: Serialize>(
        router: &Router,
        uri: impl AsRef<str> + 'async_recursion,
        body: &T,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        let request = Request::builder()
            .method(Method::POST)
            .uri(uri.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_vec(body)
                    .expect("failed to serialize POST body")
                    .into(),
            )
            .expect("failed to build POST request");
        send_request(router, request).await
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
