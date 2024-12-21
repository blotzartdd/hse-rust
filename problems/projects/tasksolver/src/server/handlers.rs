use std::convert::Infallible;

use crate::worker_pool::worker_pool::{TaskInfo, WorkerPool};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::models::requests::{CreateTaskRequest, GetStatusRequest};
use super::models::responses::{CreateTaskResponse, GetStatusResponse, GetTaskCountResponse};
use super::server::TaskStatus;

use uuid::Uuid;

// Error for get status func
// Returns ff task with given id doesn't exist
#[derive(Debug, Clone)]
struct TaskNotExistError;

impl std::fmt::Display for TaskNotExistError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "task with that id doesn't exist!")
    }
}

/// Handler for /create_task endpoint
/// Gets create task request and push it to the task queue.
/// Creates default get status response and insert it into
/// task status hashmap by generated uuid, then return
/// response with id of task.
pub async fn create_task(
    request: CreateTaskRequest,
    worker_pool: Arc<Mutex<WorkerPool>>,
    task_status: TaskStatus,
) -> Result<CreateTaskResponse, Infallible> {
    let task_status_clone = task_status.clone();
    let task_status_hashmap = task_status.task_status_chashmap;

    let status = GetStatusResponse::new();
    let id = Uuid::new_v4().to_string();
    task_status_hashmap.insert(id.clone(), status);

    let mut worker_pool = worker_pool.lock().await;
    let task_info = TaskInfo::new(id.to_string(), request, task_status_clone);
    worker_pool.do_task(task_info).await;

    let response = CreateTaskResponse { id };

    Ok(response)
}

/// Handler for /get_status endpoint
/// Gets get status request and fetch task status
/// by that id. If that id doesn't exist, return json with
/// error message.
pub async fn get_status(
    request: GetStatusRequest,
    task_status: TaskStatus,
) -> Result<GetStatusResponse, Infallible> {
    let id = request.id;
    let task_status_hashmap = task_status.task_status_chashmap.clone();

    let status = task_status_hashmap.get(&id);
    let result = Ok(status.unwrap().clone());
    result
}

/// Handler for /get_task_count endpoint
/// Returns amount of tasks in task queue
pub async fn get_task_count(
    worker_pool: Arc<Mutex<WorkerPool>>,
) -> Result<GetTaskCountResponse, Infallible> {
    let sender = worker_pool.lock().await.sender.clone();
    let response = GetTaskCountResponse {
        tasks: sender.len(),
    };

    Ok(response)
}
