use actix_web::{get, post, web, Error, HttpResponse, http::StatusCode};
use anyhow::Result;
use sqlx::SqlitePool;
use crate::joke::Joke;

#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world, Rust!"))
}

#[get("/jokes")]
pub async fn all_jokes(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(Joke::all(pool)
       .await
       .map(|some_jokes| HttpResponse::Ok().json(some_jokes))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/jokes/{joke_id}")]
pub async fn get_joke(pool: web::Data<SqlitePool>, web::Path(joke_id): web::Path<i64>) -> Result<HttpResponse, Error>{
    Ok(Joke::get(pool, joke_id)
       .await
       .map(|joke| HttpResponse::Ok().json(joke))
       .map_err(|_| HttpResponse::InternalServerError())?)
}
