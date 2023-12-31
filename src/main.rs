use crate::{
    api::auth::{
        create_admin_user, create_session_resources, delete_user, login, register, Backend,
    },
    front::{
        site::{admin_page, create_asset_dir_service, map_page},
        users::users,
    },
};
use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    http::{StatusCode, header::{X_FRAME_OPTIONS, STRICT_TRANSPORT_SECURITY, CACHE_CONTROL}, HeaderValue, HeaderMap},
    routing::{delete, get, post},
    BoxError, Extension, Router, Server,
};
use axum_csrf::{CsrfConfig, CsrfLayer};
use axum_login::{login_required, permission_required, AuthManagerLayer};
use front::{
    auth::{login_page, register_page, root_page},
    site::user_page,
};
use sqlx::PgPool;
use tower_default_headers::DefaultHeadersLayer;
use std::{net::SocketAddr, str::FromStr};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod database;
mod front;

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

    let config = CsrfConfig::default();

    let query_pool = PgPool::connect(DATABASE_URL).await?;
    create_admin_user(&query_pool)
        .await
        .inspect_err(|err| println!("These error suggests two consecutive instances run with the same admin password \n {:?}", err))
        .ok();

    let asset_service = create_asset_dir_service();

    let (backend, session_layer) = create_session_resources().await;
    let auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            println!("bad request: error: {}", err);
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayer::new(backend, session_layer));

    let mut default_headers = HeaderMap::new();
    default_headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("deny"));
    default_headers.insert(STRICT_TRANSPORT_SECURITY, HeaderValue::from_static("max-age=31536000"));
    default_headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));

    let app = Router::new()
        .route("/admin", get(admin_page))
        .route("/admin/users", get(users))
        .route("/api/delete_user", delete(delete_user))
        .route_layer(permission_required!(Backend, login_url = "/login", "admin"))
        .route("/users", get(user_page))
        .route("/map", get(map_page))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/", get(root_page))
        .route("/auth/register", post(register))
        .route("/register", get(register_page))
        .route("/login", get(login_page).post(login))
        .nest_service("/assets", asset_service)
        .layer(Extension(query_pool))
        .layer(auth_service)
        .layer(CsrfLayer::new(config))
        .layer(DefaultHeadersLayer::new(default_headers))
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    let socket_addr = SocketAddr::from_str("0.0.0.0:3000")?;

    println!(
        "[Ester Egg - kuba nie nawidze cie za bugi <3] Listening on {}",
        socket_addr
    );

    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
