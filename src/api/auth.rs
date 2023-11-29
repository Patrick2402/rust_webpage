use crate::DATABASE_URL;
use crate::{api::NextPage, database::models::User};
use anyhow::Result;
use askama_axum::IntoResponse;
use async_trait::async_trait;
use axum::{extract::Query, http::StatusCode, response::Redirect, Extension, Form};
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Connection, PgPool, Pool, Postgres};
use std::collections::HashSet;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

// authentication backend
#[derive(Clone)]
pub struct Backend {
    pool: PgPool,
}

impl Backend {
    pub async fn new() -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: PgPool::connect(DATABASE_URL).await?,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        Credentials { username, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        println!("{:?}, {:?}", username, password);

        let user = sqlx::query_as!(User, "select * from users where username = $1", username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(match user {
            Some(user) if user.password_hash == password => Some(user),
            _ => None,
        })
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        sqlx::query_as!(User, "select * from users where id = $1", user_id)
            .fetch_optional(&self.pool)
            .await
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Creds {
    username: String,
    password: String,
}

pub async fn create_session_resources() -> (Backend, SessionManagerLayer<MemoryStore>) {
    (
        Backend::new()
            .await
            .map_err(|err| println!("ERROR: cannot establish db connection: {err}"))
            .unwrap(),
        SessionManagerLayer::new(MemoryStore::default())
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1))),
    )
}

#[debug_handler]
pub async fn login(
    Query(next_page): Query<NextPage>,
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user =
        match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => return Redirect::to("/login").into_response(),
            Err(err) => {
                println!("user login error: {}", err);
                return Redirect::to("/login").into_response();
            }
        };

    if auth_session.login(&user).await.is_err() {
        return Redirect::to("/login").into_response();
    }

    let path = next_page.next.unwrap_or("/map".to_string());

    Redirect::to(&path).into_response()
}

#[debug_handler]
pub async fn register(
    auth_session: AuthSession,
    Form(Credentials { username, password }): Form<Credentials>,
) -> impl IntoResponse {
    let default_group_id: i32 = 2; // alias normal_user
    let transaction_result = &auth_session
        .backend
        .pool
        .acquire()
        .await
        .unwrap()
        .transaction::<_, _, sqlx::Error>(|tx| {
            Box::pin(async move {
                let user = sqlx::query!(
                    "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
                    username,
                    password
                )
                .fetch_one(&mut **tx)
                .await?;

                sqlx::query!(
                    "INSERT INTO user_groups (user_id, group_id) VALUES ($1, $2)",
                    user.id,
                    default_group_id
                )
                .execute(&mut **tx)
                .await?;

                Ok(user)
            })
        })
        .await;

    match transaction_result {
        Ok(user) => {
            println!("User registered successfully with ID: {}", user.id);
            Redirect::to("/login").into_response()
        }
        Err(err) => {
            eprintln!("Error registering user: {err}");
            Redirect::to("/register").into_response()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username {
    username: String,
}

#[debug_handler]
pub async fn delete_user(
    _auth_session: AuthSession,
    Extension(pool): Extension<Pool<Postgres>>,
    Query(Username { username }): Query<Username>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("hi there");

    sqlx::query_as!(User, "select * from users where username = $1", username)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let rows_affected = sqlx::query!("DELETE FROM users WHERE username = $1", username)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK.into_response())
}

#[async_trait]
impl AuthzBackend for Backend {
    type Permission = String;

    async fn get_user_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        // Implement logic to retrieve user-specific permissions from the database
        let permissions: Vec<String> = sqlx::query!(
            "SELECT p.group_name FROM user_groups ug
            INNER JOIN permissions p ON ug.group_id = p.group_id
            WHERE ug.user_id = $1",
            user.id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| row.group_name)
        .collect();

        Ok(HashSet::from_iter(permissions))
    }

    // async fn get_group_permissions(
    //     &self,
    //     _user: &Self::User,
    // ) -> Result<HashSet<Self::Permission>, Self::Error> {
    //     // Implement logic to retrieve group-specific permissions from the database
    //     let permissions: Vec<String> = sqlx::query!("SELECT group_name FROM permissions")
    //         .fetch_all(&self.pool)
    //         .await?
    //         .into_iter()
    //         .map(|row| row.group_name)
    //         .collect();

    //     Ok(HashSet::from_iter(permissions))
    // }
}

const ADMIN_PASS: &str = dotenv!("ADMIN_PASS");

pub async fn create_admin_user(pool: &PgPool) -> anyhow::Result<()> {
    struct Id {
        id: i32,
    }

    let id =
        query_as!(
            Id,
            r#"
        INSERT INTO users (id, username, password_hash)
        VALUES (
            DEFAULT,
            'admin',
            $1
        )
        ON CONFLICT (username) DO UPDATE
        SET
            password_hash = EXCLUDED.password_hash
        RETURNING id
        "#,
            ADMIN_PASS
        )
        .fetch_one(pool)
        .await?;

    query!(
        r#"
        INSERT INTO user_groups (user_id, group_id)
        VALUES ($1, (SELECT group_id FROM permissions WHERE group_name = 'admin'))
        "#,
        id.id
    )
    .execute(pool)
    .await?;

    Ok(())
}
