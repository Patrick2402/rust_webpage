use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    BoxError, Router, Server,
};
use axum_login::{login_required, AuthManagerLayer};
use front::{
    auth::{login_page, register_page, root_page},
    site::{test_page, user_page},
};
use std::{net::SocketAddr, path::PathBuf, str::FromStr};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod database;
mod front;

use crate::api::auth::{create_session_resources, login, register, Backend};

#[macro_use]
extern crate dotenv_codegen;

pub const DATABASE_URL: &str = dotenv!("DATABASE_URL");

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let asset_service = ServeDir::new(
        [std::env::current_dir()?.to_str().unwrap(), "assets"]
            .iter()
            .collect::<PathBuf>(),
    );

    let (backend, session_layer) = create_session_resources().await;
    let auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            println!("bad request: error: {}", err);
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayer::new(backend, session_layer));

    let app = Router::new()
        .route("/test", get(test_page))
        .route("/users", get(user_page))
        // protects upwards meaning that the routes that are before that
        // are protected by authentication service
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/", get(root_page))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/register", get(register_page))
        .route("/login", get(login_page))
        .nest_service("/assets", asset_service)
        .layer(auth_service)
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    let socket_addr = SocketAddr::from_str("0.0.0.0:3000")?;

    println!("[Info] Listening on {}", socket_addr);

    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
