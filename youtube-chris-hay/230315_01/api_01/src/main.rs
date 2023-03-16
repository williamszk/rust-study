#![allow(unused)]

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
// Actix

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hi! Server started at localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/greet", web::get().to(greet))
            .route("/greet/{name}", web::get().to(greet))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn hello(req: HttpRequest) -> impl Responder {
    format!("Hello World, from an Actix server.")
}
