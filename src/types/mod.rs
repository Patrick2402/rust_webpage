use axum::{response::IntoResponse, routing::get, Extension, Router};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, AuthUser, RequireAuthorizationLayer, SqliteStore,
    secrecy::{Secret, SecretVec}};
use rand::Rng;
use sqlx::sqlite::SqlitePoolOptions;

pub type Id = i64;

#[derive(Debug, Default, Clone, sqlx::FromRow)]
pub struct User { 
    pub id: Id,
    pub password_hash: String,
}


impl AuthUser<Id> for User {
    fn get_id(&self) -> Id {
        self.id
    }

    fn get_password_hash(&self) -> axum_login::secrecy::SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}
