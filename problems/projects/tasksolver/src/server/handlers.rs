use std::convert::Infallible;

use super::models::requests::{CreateTaskRequest, GetStatusRequest};
use super::models::responses::{CreateTaskResponse, GetStatusResponse, GetTaskCountResponse};
use super::server_structures::{TaskQueue, TaskStatus};

use uuid::Uuid;
use warp;

/// Handler for /create_task endpoint
/// Gets create task request and push it to the task queue.
/// Creates default get status response and insert it into 
/// task status hashmap by generated uuid, then return
/// response with id of task.
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

/// Handler for /get_status endpoint
/// Gets get status request and fetch task status
/// by that id. If that id doesn't exist, return json with
/// error message.
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

/// Handler for /get_task_count endpoint
/// Returns amount of tasks in task queue
pub async fn get_task_count(queue_task: TaskQueue) -> Result<impl warp::Reply, Infallible> {
    let queue = queue_task.lock().await;
    let response = GetTaskCountResponse { tasks: queue.len() };

    Ok(warp::reply::json(&response))
}

#[cfg(test)]
mod test_create_task_handler {
    use crate::server::models::requests::CreateTaskRequest;
    use crate::server::server_structures::{TaskQueue, init_task_queue, TaskStatus, init_task_status};
    use crate::server::handlers::create_task;
    use tokio;

    #[tokio::test] 
    async fn test_create_task() {
        let request = CreateTaskRequest {
            r#type: "bin".to_string(),
            file: "echo Hello, world!".to_string(),
            args: "".to_string(),
        };

        let task_queue = init_task_queue();
        let task_status = init_task_status();


        let cloned_task_queue = task_queue.clone();
        let cloned_task_status = task_status.clone();
        assert_eq!(cloned_task_queue.lock().await.len(), 0);
        assert_eq!(cloned_task_status.lock().await.len(), 0);
        drop(cloned_task_queue);
        drop(cloned_task_status);

        let result = create_task(request, task_queue.clone(), task_status.clone()).await;
        let task_queue = task_queue.lock().await;
        let task_status = task_status.lock().await;

        assert_eq!(task_queue.len(), 1);
        assert_eq!(task_status.len(), 1);
        assert_eq!(result.is_ok(), true);
    }
}
