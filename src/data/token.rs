use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{migrate, postgres::PgPoolOptions, FromRow, Pool, Postgres};

#[derive(sqlx::FromRow)]
pub struct Token {
    pub id: String,
    pub expired_at: chrono::DateTime<chrono::Utc>,
}
