use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let default_port = 8081;
    let worker_port: i32;
    if args.len() > 1 {
        let port: Result<i32, _> = args[1].parse();
        match port {
            Ok(parsed_num) => {
                worker_port = parsed_num;
            }
            Err(err) => {
                println!("{}", err);
                panic!(
                    "Sorry, we had a problem parsing the 'port' argument, it should be a number."
                );
            }
        }
    } else {
        worker_port = default_port;
    }

    println!("Worker container server running on port {}", worker_port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8081))? // The first argument here should be domain of the server like: "127.0.0.1"
    .run()
    .await
}

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! from a worker node.")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
