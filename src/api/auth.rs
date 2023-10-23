use axum::async_trait;
use axum::{
    extract::{FromRequest, State},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Form, Json, RequestExt,
};
use axum_login::SqliteStore;
use axum_login::axum_sessions::SessionLayer;
use axum_login::axum_sessions::async_session::MemoryStore;
use dashmap::DashMap;
use hyper::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub use crate::types::{Id, User};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Creds {
    username: String,
    password: String,
}

// TODO: refactor pool as some state or other better mathod
pub async fn login(
    State(state): State<Arc<LoginDbMockup>>,
    mut auth: AuthContext,
    JsonOrForm(creds): JsonOrForm<Creds>,
) -> StatusCode {

    let Some(value) = state.get(&creds.username) else {
        return StatusCode::UNAUTHORIZED
    };

    if *value == creds.password {
        StatusCode::OK
    } else {
        StatusCode::UNAUTHORIZED
    }
}

pub type LoginDbMockup = DashMap<String, String>;

#[axum_macros::debug_handler]
pub async fn register(
    State(db): State<Arc<LoginDbMockup>>,
    JsonOrForm(creds): JsonOrForm<Creds>,
) -> Result<(), StatusCode> {
    if db.contains_key(&creds.username) {
        Err(StatusCode::IM_USED)
    } else {
        db.insert(creds.username, creds.password);
        Ok(())
    }
}

pub type AuthContext = axum_login::extractors::AuthContext<Id, User, SqliteStore<User>>;