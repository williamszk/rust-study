
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Make a request to a worker
    let body = reqwest::get("http://dustr-worker:8081")
    .await
    .expect("Sorry, we couldn't connect to the server.")
    .text()
    .await
    .expect("Sorry, we had a problem getting the response of the request.");

    println!("{:#?}", body);

    println!("Main node server running on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

