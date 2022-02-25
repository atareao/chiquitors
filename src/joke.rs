use actix_web::web;
use chrono::NaiveDateTime;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error, Row};
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Joke{
    pub id: i64,
    pub author: String,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

    pub async fn min(pool: &web::Data<SqlitePool>) -> Result<i64, Error>{
        let row: (i64, ) = query_as(r#"SELECT MIN(id) FROM jokes"#)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(row.0)
    }

    pub async fn max(pool: &web::Data<SqlitePool>) -> Result<i64, Error>{
        let row: (i64, ) = query_as(r#"SELECT MAX(id) FROM jokes"#)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(row.0)
    }

    pub async fn random(pool: web::Data<SqlitePool>) -> Result<Joke, Error>{
        let min = Self::min(&pool).await?;
        let max = Self::max(&pool).await?;
        let id: i64 = rand::thread_rng().gen_range::<i64, _>(min..max);
        Self::get(pool, id)
            .await
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
