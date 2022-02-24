mod routes;
mod joke;

use sqlx::sqlite::SqlitePoolOptions;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use crate::routes::{root, all_jokes, get_joke};

// use actix_web::{App, HttpServer, web};
// use crate::routes::{root, all_jokes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database not found");
    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .expect("pool failed");
    HttpServer::new(move ||{
        App::new()
            .data(pool.clone())
            .service(root)
            .service(all_jokes)
            .service(get_joke)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
