use std::sync::Arc;
use tokio::sync::Mutex;
use super::models::requests::CreateTaskRequest;
use super::models::responses::GetStatusResponse;
use std::collections::HashMap;

pub type TaskQueue = Arc<Mutex<Vec<CreateTaskRequest>>>;

pub fn init_task_queue() -> TaskQueue {
    Arc::new(Mutex::new(Vec::new()))
}

pub type TaskStatus = Arc<Mutex<HashMap<String, GetStatusResponse>>>;

pub fn init_task_status() -> TaskStatus {
    Arc::new(Mutex::new(HashMap::new()))
}
