use crate::file_executer::file_executer::{binary_execute, python_execute};
use crate::server::models::requests::CreateTaskRequest;
use crate::server::server_structures::TaskStatus;
use chrono::prelude::*;

use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;

pub struct WorkerPool {
    // pub workers: Vec<Worker>,
    pub workers_count: usize,
    pub currently_working_count: usize,
    pub sender: mpsc::Sender<(
        String,
        CreateTaskRequest,
        TaskStatus,
        Arc<Mutex<WorkerPool>>,
    )>,
    pub receiver: Arc<
        Mutex<
            mpsc::Receiver<(
                String,
                CreateTaskRequest,
                TaskStatus,
                Arc<Mutex<WorkerPool>>,
            )>,
        >,
    >,
}

impl WorkerPool {
    pub async fn new(
        workers_count: usize,
        sender: mpsc::Sender<(
            String,
            CreateTaskRequest,
            TaskStatus,
            Arc<Mutex<WorkerPool>>,
        )>,
        receiver: Arc<
            Mutex<
                mpsc::Receiver<(
                    String,
                    CreateTaskRequest,
                    TaskStatus,
                    Arc<Mutex<WorkerPool>>,
                )>,
            >,
        >,
    ) -> WorkerPool {
        // let mut workers = Vec::new();
        for i in 0..workers_count {
            create_worker(i, receiver.clone()).await;
            // workers.push(worker);
        }

        WorkerPool {
            // workers,
            workers_count,
            currently_working_count: 0,
            sender,
            receiver,
        }
    }

    pub async fn do_task(
        &mut self,
        id: &str,
        task: CreateTaskRequest,
        task_status: TaskStatus,
        worker_pool: Arc<Mutex<WorkerPool>>,
    ) {
        println!("SENDING TASK!");
        self.currently_working_count += 1;
        println!(
            "currently_working_count: {}",
            (*self).currently_working_count
        );
        let _ = self
            .sender
            .send((id.to_string(), task, task_status, worker_pool))
            .await;
    }
}

async fn create_worker(
    worker_id: usize,
    receiver: Arc<
        Mutex<
            mpsc::Receiver<(
                String,
                CreateTaskRequest,
                TaskStatus,
                Arc<Mutex<WorkerPool>>,
            )>,
        >,
    >,
) {
    task::spawn(async move {
        println!("NEW THREAD!");
        println!("Thread {} blocked receiver!", worker_id);

        loop {
            let mut receiver = receiver.lock().await;
            if let Some((id, task, task_status_hashmap, worker_pool)) = receiver.recv().await {
                println!("Task of worker {} started!", worker_id);
                let mut task_status = task_status_hashmap.lock().await;
                let status = task_status.get_mut(&id).unwrap();
                status.status = "RUNNING".to_string();
                status.meta.started_at = Some(Utc::now().to_string());

                drop(task_status);
                drop(receiver);

                let (stdout, stderr, execution_result) = match task.r#type.as_str() {
                    "python" => python_execute(task.file, task.args).await,
                    "bin" => binary_execute(id.clone(), task.file, task.args).await,
                    _ => {
                        println!("Bruh");
                        ("".to_string(), None, "".to_string())
                    }
                };

                let mut task_status = task_status_hashmap.lock().await;
                let status = task_status.get_mut(&id).unwrap();
                status.result.stdout = stdout;
                status.result.stderr = stderr;
                status.status = execution_result;
                status.meta.finished_at = Some(Utc::now().to_string());

                let mut pool = worker_pool.lock().await;
                pool.currently_working_count -= 1;

                println!("Task of worker {} finished!", worker_id);
            }
        }
    });
}
