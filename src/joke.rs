use actix_web::actix_web;
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
