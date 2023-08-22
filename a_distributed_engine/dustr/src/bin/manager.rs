use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: figure out where to use those arguments from args
    let args = Cli::parse();
    println!(">>>>>>>>>>>>> args: {:?}", args);

    // Make a request to a worker - worker 1 =============================================
    send_request_to_worker("http://dustr-worker-01:8081").await;

    // Make a request to a worker - worker 2 =============================================
    send_request_to_worker("http://dustr-worker-02:8081").await;

    // Running manager node API server =============================================
    println!("Manager node server running on port 8080");
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

/// worker_url = "http://dustr-worker-02:8081"
async fn send_request_to_worker(worker_url: &str) {
    let body = reqwest::get(worker_url)
        .await
        .expect("Sorry, we couldn't connect to the server.")
        .text()
        .await
        .expect("Sorry, we had a problem getting the response of the request.");

    println!(">>> Received response from worker: {:#?}", body);
}

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

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
