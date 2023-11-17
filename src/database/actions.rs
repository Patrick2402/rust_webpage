use diesel::ExpressionMethods;
use diesel::{query_dsl::QueryDsl, PgConnection, RunQueryDsl, SelectableHelper};

use super::models::{NewUser, User};
use super::schema::users;

/// diesel util function we might no need that like ever
pub(crate) fn _create_user(conn: &mut PgConnection, username: &str, password_hash: &str) -> User {
    let user = NewUser {
        username,
        password_hash,
    };

    diesel::insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error adding user to db")
}

/// diesel util function we might no need that like ever
pub(crate) fn _fetch_user(
    conn: &mut PgConnection,
    username: &str,
) -> Result<Vec<User>, diesel::result::Error> {
    users::table
        .filter(users::username.eq(username))
        .load::<User>(conn)
}
