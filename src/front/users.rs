use crate::database::models::User;
use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum_login::AuthUser;
use sqlx::{query_as, PgPool};

#[derive(Debug)]
struct UserInPanel {
    id: <User as AuthUser>::Id,
    username: String,
    role: String,
}

#[derive(Template)]
#[template(path = "users.html")]
struct UserTable {
    users: Vec<UserInPanel>,
}

pub async fn users(Extension(ref pool): Extension<PgPool>) -> impl IntoResponse {
    let users =
        query_as!(
            UserInPanel,
            r#"
            SELECT u.id,u.username, p.group_name AS role
                FROM users u
                JOIN user_groups ug ON u.id = ug.user_id
                JOIN permissions p ON ug.group_id = p.group_id
            "#
        )
        .fetch_all(pool)
        .await
        .unwrap();

    UserTable { users }.into_response()
}
