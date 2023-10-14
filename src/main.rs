mod auth;
mod site;

use anyhow::Result;
use auth::login;
use axum::{routing::get, Router};
use hyper::Server;
use site::{test_page, user};
use std::{net::SocketAddr, str::FromStr};
use tower_http::{trace::TraceLayer, services::ServeDir};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let jump = get(|| async { "Just trying to compile some good rust and earn money!" });

    let app = Router::new()
        .route("/", get(jump))
        .route("/test", get(test_page))
        .route("/users", get(user))
        .route("/login", get(login))
        .nest_service("/assets", ServeDir::new("./assets"))
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    let socket_addr = SocketAddr::from_str("127.0.0.1:3000")?;

    println!("[Info] Listening on {}", socket_addr);

    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
