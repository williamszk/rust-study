use crate as dustr;

pub async fn main_01() {
    // Make a request to a worker - worker 1 ===================================
    _send_request_to_worker("http://dustr-worker-01:8081").await;

    // Make a request to a worker - worker 2 ===================================
    _send_request_to_worker("http://dustr-worker-02:8081").await;

    // Make a request to a worker with a specific body =========================
    _deal_with_request_that_manager_sent().await;

    // experiments ===================================================
    _experiment_wrapper().await;
    // ========================================================================
}

async fn _experiment_wrapper() {
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
async fn _send_request_to_worker(worker_url: &str) {
    let res_body = reqwest::get(worker_url)
        .await
        .expect("Sorry, we couldn't connect to the server.")
        .text()
        .await
        .expect("Sorry, we had a problem getting the response of the request.");

    println!(">>> Received response from worker: {:#?}", res_body);
}

async fn _send_request_to_worker_with_body(worker_url: &str) -> reqwest::Result<reqwest::Response> {
    let client = reqwest::Client::new();
    let res_body = client
        .post(worker_url)
        .body("This is the body that the manager sent")
        .send()
        .await;
    // println!(">>> Received response from worker: {:#?}", res_body);
    return res_body;
}

async fn _deal_with_request_that_manager_sent() {
    let res_result =
        _send_request_to_worker_with_body("http://dustr-worker-02:8081/double_map").await;

    match res_result {
        Ok(res) => match res.text().await {
            Ok(res2) => {
                println!("res2: '{:}'", res2);
            }
            Err(error) => {
                println!("error: '{:}'", error);
            }
        },
        Err(error) => {
            println!("error: '{:}'", error);
        }
    }
}
