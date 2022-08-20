use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{migrate, postgres::PgPoolOptions, FromRow, Pool, Postgres};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    pub name: String,
    pub age: i32,
}
