use std::{thread, time::Duration};

pub async fn _send_struct_to_worker() {
    use crate::core::manager::send_payload::payload::PayloadVectorInt32;

    let a_payload = PayloadVectorInt32 {
        data: vec![1, 2, 3, 4, 5, 6, 7],
    };

    let worker_url = "http://dustr-worker-01:8081/echo";
    thread::sleep(Duration::from_millis(2000));

    let client = reqwest::Client::new();
    let res_body = client.post(worker_url).json(&a_payload).send().await;
    match res_body {
        Ok(response) => {
            let res01 = response.text().await;
            println!(
                "The response from the echo endpoint: {:?}",
                res01.expect("Sorry")
            );
        }
        Err(error) => println!("{}", error),
    };
}

pub async fn _send_map_to_worker() {
    // [x] Manager send an array to worker

    let worker_url = "http://dustr-worker-01:8081/echo";

    use std::collections::HashMap;
    let mut _map = HashMap::new();
    _map.insert("lang", "rust");
    _map.insert("body", "json");
    let req_body = _map;

    thread::sleep(Duration::from_millis(2000));

    let client = reqwest::Client::new();
    let res_body = client.post(worker_url).json(&req_body).send().await;
    match res_body {
        Ok(response) => {
            let res01 = response.text().await;
            println!(
                "The response from the echo endpoint: {:?}",
                res01.expect("Sorry")
            );
        }
        Err(error) => println!("{}", error),
    };
}
