use std::convert::Infallible;

use super::models::requests::{CreateTaskRequest, GetStatusRequest};
use super::models::responses;
use super::server_structures::{TaskQueue, TaskStatus};

use serde_json;
use warp;

pub async fn create_task(request: CreateTaskRequest, queue_task: TaskQueue, task_status: TaskStatus) -> Result<impl warp::Reply, Infallible> {
    let mut queue = queue_task.lock().await;
    queue.push(request);
    println!("Doing create_task!");
    // println!("Request: {}", serde_json::to_string(&request).unwrap());
    Ok(warp::reply::with_status("success", warp::http::StatusCode::CREATED))
}

pub async fn get_status(request: GetStatusRequest, task_status: TaskStatus) -> Result<impl warp::Reply, Infallible> {
    println!("Doing get_status!");
    Ok(warp::reply::reply())
}

pub async fn get_task_count(queue_task: TaskQueue) -> Result<impl warp::Reply, Infallible> {
    let queue = queue_task.lock().await;
    let response = responses::GetTaskCountResponse {
        tasks: queue.len(),
    };

    Ok(warp::reply::json(&response))
}
