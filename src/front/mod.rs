pub mod auth;
pub mod site;

use axum::{routing::get, Router};

use self::auth::{login, register, root};
use self::site::{test_page, user};

pub fn front_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/test", get(test_page))
        .route("/users", get(user))
        .route("/register", get(register))
        .route("/login", get(login))
}