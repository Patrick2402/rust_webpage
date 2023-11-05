use anyhow::{Context, Result};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, sqlite::SqliteQueryResult, Error};
use crate::types::User;

pub async fn db_init_sqlite(url: &str) -> Result<()> {
    if !Sqlite::database_exists(url).await.unwrap_or(false) {
        Sqlite::create_database(url)
            .await
            .context("err creating the databese")?
    }

    Ok(())
}

pub async fn create_users_table(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id bigserial,
            username text,
            password_hash text
      );"#,
    )
    .execute(pool)
    .await
}

impl User {
    pub async fn db_insert(&self, pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
        sqlx::query(
            "INSERT INTO users (id, password_hash)
            VALUES ($1, $2)",
        )
        .bind(self.id)
        .bind(&self.password_hash)
        .execute(pool)
        .await
    }
}
