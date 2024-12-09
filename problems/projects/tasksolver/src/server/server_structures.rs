use std::sync::Arc;
use tokio::sync::Mutex;
use super::models::requests::CreateTaskRequest;
use super::models::responses::GetStatusResponse;
use std::collections::HashMap;

pub type TaskQueue = Arc<Mutex<Vec<CreateTaskRequest>>>;

pub fn init_task_queue() -> TaskQueue {
    Arc::new(Mutex::new(Vec::new()))
}

pub type TaskStatus = HashMap<String, GetStatusResponse>;
