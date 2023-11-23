pub(crate) mod auth;

use axum::async_trait;
use axum::http::header::CONTENT_TYPE;
use axum::{
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Form, Json, RequestExt,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NextPage {
    next: Option<String>,
}

/// Utils extractor once something talks json
pub struct JsonOrForm<T>(T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for JsonOrForm<T>
where
    S: Send + Sync,
    B: Send + 'static + std::fmt::Debug,
    Json<T>: FromRequest<(), B>,
    Form<T>: FromRequest<(), B>,
    T: 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }
            let Form(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
            return Ok(Self(payload));
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}
