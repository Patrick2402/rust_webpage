mod api;
mod front;

use anyhow::Result;
use axum::Router;
use hyper::Server;
use std::{net::SocketAddr, path::PathBuf, str::FromStr};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::api_router;
use front::front_router;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api_router = api_router();
    let front_router = front_router();
    let asset_path: PathBuf = [std::env::current_dir()?.to_str().unwrap(), "assets"]
        .iter()
        .collect();

    let app = Router::new()
        .merge(front_router)
        .merge(api_router)
        .nest_service("/assets", ServeDir::new(asset_path))
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
