mod api;
mod front;
mod types;
mod database;

use anyhow::Result;
use axum::Router;
use axum_login::{AuthLayer, SqliteStore};
use hyper::Server;
use rand::Rng;
use sqlx::sqlite::SqlitePoolOptions;
use std::{net::SocketAddr, path::PathBuf, str::FromStr};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use types::User;
use database::{db_init_sqlite, create_users_table};

use api::api_router;
use front::front_router;

const DATABASE_URL: &str = "sqlite:sqlite/user_store.db";

#[tokio::main]
async fn main() -> Result<()> {
    let secret: [u8; 64] = rand::thread_rng().gen();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // let api_router = api_router();
    // let front_router = front_router();

    let asset_path: PathBuf = [std::env::current_dir()?.to_str().unwrap(), "assets"]
        .iter()
        .collect();

    db_init_sqlite(DATABASE_URL).await?;
    let pool = SqlitePoolOptions::new()
        .connect("sqlite/user_store.db")
        .await?;

    create_users_table(&pool).await?;

    let u = User {
        id: 1,
        password_hash: "assword".to_string(),
    };

    sqlx::query(
        "INSERT INTO users (id, password_hash)
        VALUES ($1, $2)",
    )
    .bind(u.id)
    .bind(u.password_hash)
    .execute(&pool)
    .await?;

    let a: Vec<User> = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;

    dbg!(a);

    let mut user_store = SqliteStore::<User>::new(pool);
    let auth_layer = AuthLayer::new(user_store, &secret);

    // insert a generic user

    let app = Router::new()
        .nest_service("/assets", ServeDir::new(asset_path))
        .merge(front_router())
        .merge(api_router())
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
