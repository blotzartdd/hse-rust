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

    let status = GetStatusResponse::new_utc_status();
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

    if let Some(status) = task_status_hashmap.get(&id) {
        return Ok(status.clone());
    }

    let error_status = GetStatusResponse::new_error_status();
    Ok(error_status)
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

#[cfg(test)]
mod test_create_task {
    use crate::server::handlers::create_task;
    use crate::server::models::requests::{CreateTaskRequest, TaskType};
    use crate::server::models::responses::TaskStatusEnum;
    use crate::server::server::TaskStatus;
    use crate::worker_pool::worker_pool::WorkerPool;
    use base64::prelude::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_create_python_task() {
        let workers_count = 4;

        let (task_sender, task_receiver) = async_channel::unbounded();
        let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
            workers_count,
            task_sender,
            task_receiver,
        )));
        let task_status = TaskStatus::new();

        let python_code = "print('Hello, world!')".to_string();
        let arguments = "".to_string();

        let create_task_request = CreateTaskRequest::new(TaskType::Python, python_code, arguments);

        let create_task_response =
            create_task(create_task_request, worker_pool, task_status.clone())
                .await
                .unwrap();
        let id = create_task_response.id;
        let status = task_status.task_status_chashmap.get(&id);

        assert_eq!(status.is_some(), true);

        let get_status_response = status.unwrap();
        assert_eq!(get_status_response.status, TaskStatusEnum::WAIT);
    }

    #[tokio::test]
    async fn test_create_binary_task() {
        let workers_count = 4;

        let (task_sender, task_receiver) = async_channel::unbounded();
        let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
            workers_count,
            task_sender,
            task_receiver,
        )));
        let task_status = TaskStatus::new();

        let base64_encoded_file = BASE64_STANDARD.encode("echo Hello, world!");
        let arguments = "".to_string();

        let create_task_request =
            CreateTaskRequest::new(TaskType::Bin, base64_encoded_file, arguments);

        let create_task_response =
            create_task(create_task_request, worker_pool, task_status.clone())
                .await
                .unwrap();
        let id = create_task_response.id;
        let status = task_status.task_status_chashmap.get(&id);

        assert_eq!(status.is_some(), true);

        let get_status_response = status.unwrap();
        assert_eq!(get_status_response.status, TaskStatusEnum::WAIT);
    }
}

#[cfg(test)]
mod test_get_status {
    use crate::server::handlers::{create_task, get_status};
    use crate::server::models::requests::{CreateTaskRequest, GetStatusRequest, TaskType};
    use crate::server::models::responses::TaskStatusEnum;
    use crate::server::server::TaskStatus;
    use crate::worker_pool::worker_pool::WorkerPool;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_get_status_of_python_task() {
        let workers_count = 4;

        let (task_sender, task_receiver) = async_channel::unbounded();
        let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
            workers_count,
            task_sender,
            task_receiver,
        )));
        let task_status = TaskStatus::new();
        let task_status_clone = task_status.clone();

        let python_code = "print('Hello, world!')".to_string();
        let arguments = "".to_string();

        let create_task_request = CreateTaskRequest::new(TaskType::Python, python_code, arguments);

        let create_task_response =
            create_task(create_task_request, worker_pool, task_status.clone())
                .await
                .unwrap();
        let id = create_task_response.id;
        let status = task_status.task_status_chashmap.get(&id);

        assert_eq!(status.is_some(), true);

        let get_status_request = GetStatusRequest { id };
        let result = get_status(get_status_request, task_status_clone)
            .await
            .unwrap();

        assert_eq!(result.status, TaskStatusEnum::WAIT);
    }

    #[tokio::test]
    async fn test_get_status_of_not_exist_task() {
        let workers_count = 4;

        let (task_sender, task_receiver) = async_channel::unbounded();
        let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
            workers_count,
            task_sender,
            task_receiver,
        )));
        let task_status = TaskStatus::new();
        let task_status_clone = task_status.clone();
        let id = "random-UUID".to_string();

        let get_status_request = GetStatusRequest { id };
        let result = get_status(get_status_request, task_status_clone)
            .await
            .unwrap();

        assert_eq!(result.status, TaskStatusEnum::NOTEXIST);
    }
}

#[cfg(test)]
mod test_get_task_count {
    use crate::server::handlers::{create_task,  get_task_count};
    use crate::server::models::requests::{CreateTaskRequest, TaskType};
    use crate::server::server::TaskStatus;
    use crate::worker_pool::worker_pool::WorkerPool;
    use base64::prelude::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_get_queue_count_of_one_task() {
        let workers_count = 4;

        let (task_sender, task_receiver) = async_channel::unbounded();
        let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
            workers_count,
            task_sender,
            task_receiver,
        )));
        let task_status = TaskStatus::new();

        let python_code = "print('Hello, world!')".to_string();
        let arguments = "".to_string();

        let create_task_request = CreateTaskRequest::new(TaskType::Python, python_code, arguments);

        let create_task_response = create_task(
            create_task_request,
            worker_pool.clone(),
            task_status.clone(),
        )
        .await
        .unwrap();
        let id = create_task_response.id;
        let status = task_status.task_status_chashmap.get(&id);
        assert_eq!(status.is_some(), true);

        let result = get_task_count(worker_pool).await.unwrap();
        assert_eq!(result.tasks, 1);
    }

    #[tokio::test]
    async fn test_get_queue_count_with_no_task() {
        let workers_count = 4;

        let (task_sender, task_receiver) = async_channel::unbounded();
        let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
            workers_count,
            task_sender,
            task_receiver,
        )));

        let result = get_task_count(worker_pool).await.unwrap();
        assert_eq!(result.tasks, 0);
    }

    #[tokio::test]
    async fn test_get_queue_count_with_many_tasks() {
        let workers_count = 4;

        let (task_sender, task_receiver) = async_channel::unbounded();
        let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
            workers_count,
            task_sender,
            task_receiver,
        )));

        let task_status = TaskStatus::new();

        let python_code = "print('Hello, world!')".to_string();
        let arguments = "".to_string();
        let create_task_request = CreateTaskRequest::new(TaskType::Python, python_code, arguments);
        let _ = create_task(
            create_task_request,
            worker_pool.clone(),
            task_status.clone(),
        )
        .await
        .unwrap();

        let base64_encoded_file = BASE64_STANDARD.encode("echo Hello, world!");
        let arguments = "".to_string();
        let create_task_request =
            CreateTaskRequest::new(TaskType::Bin, base64_encoded_file, arguments);
        let _ = create_task(
            create_task_request,
            worker_pool.clone(),
            task_status.clone(),
        )
        .await
        .unwrap();

        let base64_encoded_file = BASE64_STANDARD.encode("bim bim");
        let arguments = "".to_string();
        let create_task_request =
            CreateTaskRequest::new(TaskType::Bin, base64_encoded_file, arguments);
        let _ = create_task(
            create_task_request,
            worker_pool.clone(),
            task_status.clone(),
        )
        .await
        .unwrap();

        let result = get_task_count(worker_pool).await.unwrap();
        assert_eq!(result.tasks, 3);
    }
}
