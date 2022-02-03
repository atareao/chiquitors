mod api;

use actix_web::{App, HttpServer, web};
use crate::api::{hello, echo, manual_hello, post};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(hello)
            .service(echo)
            .service(post)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
