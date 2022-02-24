use actix_web::web;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Joke{
    pub id: i64,
    pub author: String,
    pub value: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Joke{
    pub async fn all(pool: web::Data<SqlitePool>) -> Result<Vec<Joke>, Error>{
        let jokes = query_as!(Joke, r#"SELECT id, author, value, created_at, updated_at FROM jokes"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(jokes)
    }

    pub async fn get(pool: web::Data<SqlitePool>, id: i64) -> Result<Joke, Error>{
        let joke = query_as!(Joke, r#"SELECT id, author, value, created_at, updated_at FROM jokes WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(joke)
    }

    pub async fn new(pool: web::Data<SqlitePool>, author: &str, value: &str, created_at: i64, updated_at: i64) -> Result<Joke, Error>{
        let id = query("INSERT INTO jokes (author, value, created_at, updated_at) VALUES (?, ?, ?, ?);")
            .bind(author)
            .bind(value)
            .bind(created_at)
            .bind(updated_at)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Ok(Self::get(pool, id).await?)
    }
}
