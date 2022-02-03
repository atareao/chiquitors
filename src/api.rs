use actix_web::{post, get, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hola mundo!")
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Que pasa en casa!")
}

#[post("/post")]
async fn post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
