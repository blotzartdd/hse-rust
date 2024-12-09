use super::models::requests::CreateTaskRequest;
use super::models::responses::GetStatusResponse;
use crate::worker_pool::worker_pool::WorkerPool;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type TaskQueue = Arc<Mutex<VecDeque<(String, CreateTaskRequest)>>>;

pub fn init_task_queue() -> TaskQueue {
    Arc::new(Mutex::new(VecDeque::new()))
}

pub async fn monitor_queue(
    task_queue: TaskQueue,
    task_status: TaskStatus,
    worker_pool: Arc<Mutex<WorkerPool>>,
) {
    loop {
        let mut pool = worker_pool.lock().await;
        let mut queue = task_queue.lock().await;

        if pool.currently_working_count < pool.workers_count {
            if let Some((id, task)) = queue.pop_front() {
                pool.do_task(&id, task, task_status.clone()).await;
            }
        }
    }
}

pub type TaskStatus = Arc<Mutex<HashMap<String, GetStatusResponse>>>;

pub fn init_task_status() -> TaskStatus {
    Arc::new(Mutex::new(HashMap::new()))
}
