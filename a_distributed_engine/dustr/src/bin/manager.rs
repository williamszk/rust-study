use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use dustr;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // TODO: figure out where to use those arguments from args
    let _args = Cli::parse();
    // println!(">>>>>>>>>>>>> args: {:?}", args);

    // Make a request to a worker - worker 1 =============================================
    send_request_to_worker("http://dustr-worker-01:8081").await;

    // Make a request to a worker - worker 2 =============================================
    send_request_to_worker("http://dustr-worker-02:8081").await;

    // let's try to make the workers do the processing
    let res_result =
        send_request_to_worker_with_body("http://dustr-worker-02:8081/double_map").await;
    match res_result {
        Ok(res) => match res.text().await {
            Ok(res2) => {
                println!("res2: {:}", res2);
            }
            Err(error) => {
                println!("error: {}", error);
            }
        },
        Err(error) => {
            println!("error: {:}", error);
        }
    }

    // experiments ===================================================
    _experiment().await;
    // ========================================================================

    // Running manager node API server ========================================
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

async fn _experiment() {
    use dustr::experiments::project;
    let res = project::_experiment_with_sqlite();
    println!("{:?}", res);

    let _a_struct = project::ExperimentStruct {
        name: String::from("Bob"),
    };
}

fn _map_double_values() -> Vec<i32> {
    let my_arr = vec![1, 2, 3, 4];
    let squared: Vec<i32> = my_arr.iter().map(|x| x * x).collect();
    squared
}

/// Example
/// ```
/// worker_url = "http://dustr-worker-02:8081"
/// ```
async fn send_request_to_worker(worker_url: &str) {
    let res_body = reqwest::get(worker_url)
        .await
        .expect("Sorry, we couldn't connect to the server.")
        .text()
        .await
        .expect("Sorry, we had a problem getting the response of the request.");
    println!(">>> Received response from worker: {:#?}", res_body);
}

async fn send_request_to_worker_with_body(worker_url: &str) -> reqwest::Result<reqwest::Response> {
    let client = reqwest::Client::new();
    let res_body = client
        .post(worker_url)
        .body("This is the body that the manager sent")
        .send()
        .await;
    // println!(">>> Received response from worker: {:#?}", res_body);
    return res_body;
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

#[cfg(test)]
mod test {
    #[test]
    fn test_01() {
        assert!(true);
    }

    #[test]
    fn test_02() {
        let is_this_test_02 = true;
        assert!(is_this_test_02);
    }
}
