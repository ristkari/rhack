// use std::async_iter;

use crate::data::person::Person;
use crate::StdErr;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn connect() -> Result<Self, StdErr> {
        let db_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(25)
            .min_connections(5)
            .connect(&db_url)
            .await?;
        Ok(Db { pool })
    }

    pub async fn persons(&self, id: String) -> Result<Vec<Person>, StdErr> {
        let persons = sqlx::query_as("SELECT * FROM persons as p where p.id = $1")
            .bind(id)
            .fetch_all(&self.pool)
            .await?;
        Ok(persons)
    }

    pub async fn create_person(&self, person: Person) -> Result<Person, StdErr> {
        let person =
            sqlx::query_as("INSERT INTO persons (id, name, age) VALUES ($1, $2, $3) RETURNING *")
                .bind(person.id)
                .bind(person.name)
                .bind(person.age)
                .fetch_one(&self.pool)
                .await?;
        Ok(person)
    }

    pub async fn get_all_persons(&self) -> Result<Vec<Person>, StdErr> {
        let persons = sqlx::query_as("SELECT * FROM persons")
            .fetch_all(&self.pool)
            .await?;
        Ok(persons)
    }
}
