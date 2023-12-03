use super::schema::permissions;
use crate::database::schema::{sessions, users};
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSessionId {
    pub id: i32,
    pub session_id: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub session_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Permission {
    pub group_id: i32,
    pub group_name: String,
}
