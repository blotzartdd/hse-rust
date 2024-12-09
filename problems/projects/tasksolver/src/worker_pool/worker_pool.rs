use crate::server::models::requests::CreateTaskRequest;
use crate::server::server_structures::TaskStatus;
use chrono::prelude::*;

use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::Mutex;
use tokio::task;

pub struct WorkerPool {
    pub workers: Vec<Worker>,
    pub workers_count: usize,
    pub currently_working_count: usize,
    pub sender: mpsc::Sender<CreateTaskRequest>,
    pub receiver: Arc<Mutex<mpsc::Receiver<CreateTaskRequest>>>,
}

pub struct Worker {
    worker_thread: JoinHandle<()>,
}

impl WorkerPool {
    pub fn new(
        workers_count: usize,
        sender: mpsc::Sender<CreateTaskRequest>,
        receiver: Arc<Mutex<mpsc::Receiver<CreateTaskRequest>>>,
    ) -> WorkerPool {
        let mut workers = Vec::new();
        for i in 0..workers_count {
            let worker = create_worker(i, receiver.clone());
            workers.push(worker);
        }

        WorkerPool {
            workers,
            workers_count,
            currently_working_count: 0,
            sender,
            receiver,
        }
    }

    pub async fn do_task(&mut self, id: &str, task: CreateTaskRequest, task_status: TaskStatus) {
        (*self).currently_working_count += 1;

        let mut task_status_hashmap = task_status.lock().await;
        let status = task_status_hashmap.get_mut(id).unwrap();
        status.status = "RUNNING".to_string();
        status.meta.started_at = Some(Utc::now().to_string());

        let _ = (*self).sender.send(task).await;
        (*self).currently_working_count -= 1;
    }
}

fn create_worker(id: usize, receiver: Arc<Mutex<mpsc::Receiver<CreateTaskRequest>>>) -> Worker {
    let worker_thread = task::spawn(async move { 
        let mut receiver = receiver.lock().await;
        while let Some(task) = receiver.recv().await {
            println!("Task: {}", task.r#type);
            println!("Thread id: {}", id);
        }
    });

    Worker { worker_thread }
}
