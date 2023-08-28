use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Get command line arguments
    // let args: Vec<String> = env::args().collect();
    let args = Cli::parse();

    let default_port = "8081";
    let worker_name = args.worker_name;

    println!(
        "Worker container server running - worker name: '{}'",
        worker_name
    );

    let addrs = format!("0.0.0.0:{}", default_port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: worker_name.clone(),
            }))
            .service(hello_from_worker)
            .service(echo)
            .service(double_map)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(addrs)?
    .run()
    .await
}

#[derive(Debug, Parser)]
struct Cli {
    worker_name: String,
}

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello_from_worker(data: web::Data<AppState>) -> impl Responder {
    let worker_name = &data.app_name;
    HttpResponse::Ok().body(format!(
        "Hello world! from a worker node. '{}'",
        worker_name
    ))
}

#[post("/double_map")]
async fn double_map(req_body: String) -> impl Responder {
    println!("The worker received the request from manager on the endpoint '/double_map'");
    HttpResponse::Ok().body(req_body)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
