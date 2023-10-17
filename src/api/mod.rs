pub mod auth;

use std::sync::Arc;

use auth::{login, register, LoginDbMockup};
use axum::{routing::post, Router};

pub fn api_router() -> Router {
    let state = Arc::new(LoginDbMockup::new());

    Router::new()
        .route("/auth/register", post(register).with_state(state.clone()))
        .route("/auth/login", post(login).with_state(state.clone()))
}
