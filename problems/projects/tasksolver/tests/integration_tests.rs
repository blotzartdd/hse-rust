use tokio;
use tasksolver::server::server::{kill_server, run};
use tasksolver::server::models::requests::*;
use tasksolver::server::models::responses::*;
use base64::prelude::*;
use reqwest::blocking::Client;

fn build_server_url(address: &str, port: u16, endpoint: &str) -> String {
    format!("http://{}:{}/{}", address, port, endpoint)
}

#[tokio::test]
async fn it_works() {
    let address = "127.0.0.1";
    let port = 8080;
    let (server, queue_handler) = run(4, address, port).await;

    let client = Client::new();
    let create_task_url = build_server_url(address, port, "/create_task");
    let get_status_url = build_server_url(address, port, "/get_status");
    let get_task_count_url = build_server_url(address, port, "/get_task_count");

    let request = CreateTaskRequest {
        r#type: "bin".to_string(),
        file: BASE64_STANDARD.encode("echo Hello, world!").to_string(),
        args: "".to_string(),
    };

    let response = client.post(&create_task_url).json(&request).send();
    assert_eq!(response.is_ok(), true);

    let response_data: CreateTaskResponse = response.unwrap().json().unwrap();
    let id = response_data.id;

    let get_status_request = GetStatusRequest {
        id,
    };

    let response = client.get(&get_status_url).json(&get_status_request).send();
    assert_eq!(response.is_ok(), true);

    let response_data: GetStatusResponse = response.unwrap().json().unwrap();
    let status = response_data.status;
    let stdout = response_data.result.stdout;
    assert_eq!(status, "SUCCESS".to_string());
    assert_eq!(stdout, "Hello, world!".to_string());

    kill_server(server, queue_handler);
}
