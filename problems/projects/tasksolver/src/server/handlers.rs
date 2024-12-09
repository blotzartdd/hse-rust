use std::convert::Infallible;

use super::models::requests::{CreateTaskRequest, GetStatusRequest};
use super::models::responses::{CreateTaskResponse, GetStatusResponse, GetTaskCountResponse};
use super::server_structures::{TaskQueue, TaskStatus};

use uuid::Uuid;
use warp;

pub async fn create_task(
    request: CreateTaskRequest,
    queue_task: TaskQueue,
    task_status: TaskStatus,
) -> Result<impl warp::Reply, Infallible> {
    let mut queue = queue_task.lock().await;
    let mut task_status_hashmap = task_status.lock().await;

    let status = GetStatusResponse::new();
    let id = Uuid::new_v4().to_string();

    queue.push_back((id.clone(), request));
    task_status_hashmap.insert(id.clone(), status);

    let response = CreateTaskResponse { id };

    Ok(warp::reply::json(&response))
}

pub async fn get_status(
    request: GetStatusRequest,
    task_status: TaskStatus,
) -> Result<impl warp::Reply, Infallible> {
    let id = request.id;
    let task_status_hashmap = task_status.lock().await;

    if let Some(status) = task_status_hashmap.get(&id) {
        return Ok(warp::reply::json(status));
    }

    let error = "There is no task with that id.".to_string();
    Ok(warp::reply::json(&error))
}

pub async fn get_task_count(queue_task: TaskQueue) -> Result<impl warp::Reply, Infallible> {
    let queue = queue_task.lock().await;
    let response = GetTaskCountResponse { tasks: queue.len() };

    Ok(warp::reply::json(&response))
}
