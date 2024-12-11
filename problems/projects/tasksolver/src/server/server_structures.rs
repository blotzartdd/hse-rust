use super::models::requests::CreateTaskRequest;
use super::models::responses::GetStatusResponse;
use crate::worker_pool::worker_pool::WorkerPool;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Task queue from where threads will take their tasks
pub type TaskQueue = Arc<Mutex<VecDeque<(String, CreateTaskRequest)>>>;

/// Initialize task queue
pub fn init_task_queue() -> TaskQueue {
    Arc::new(Mutex::new(VecDeque::new()))
}

/// Function for monitoring task queue and doing task when 
/// free worker exist. This function being used in one of two
/// main threads in server.rs
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
                println!("Getting task from queue!");
                pool.do_task(&id, task, task_status.clone(), worker_pool.clone())
                    .await;
            }
        }
    }
}

/// Task status hashmap for all tasks on server
pub type TaskStatus = Arc<Mutex<HashMap<String, GetStatusResponse>>>;

/// Initialize task status hashmap
pub fn init_task_status() -> TaskStatus {
    Arc::new(Mutex::new(HashMap::new()))
}
