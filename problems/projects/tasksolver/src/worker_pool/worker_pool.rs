use crate::file_executer::file_executer::{binary_execute, python_execute};
use crate::server::models::requests::CreateTaskRequest;
use crate::server::server_structures::TaskStatus;
use chrono::prelude::*;

use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task;

// Struct of tokio threads that will be taking tasks from sender and execute them.
pub struct WorkerPool {
    // pub workers: Vec<Worker>,
    // Amount of threads in thread pool
    workers_count: usize,
    // Amount of threads, that working on tasks right now
    pub currently_working_count: usize,
    // Tokio sender that sends task id, task request, task status hashmap and current worker pool
    pub sender: async_channel::Sender<(
        String,
        CreateTaskRequest,
        TaskStatus,
        Arc<Mutex<WorkerPool>>,
    )>,
    // Tokio receiver that recieve task id, task request, task status hashmap and current worker pool
    pub receiver: 
            async_channel::Receiver<(
                String,
                CreateTaskRequest,
                TaskStatus,
                Arc<Mutex<WorkerPool>>,
            )>,
}

impl WorkerPool {
    /// Creates WorkerPool struct with given amount of workes, sender and receiver.
    pub fn new(
        workers_count: usize,
        sender: async_channel::Sender<(
            String,
            CreateTaskRequest,
            TaskStatus,
            Arc<Mutex<WorkerPool>>,
        )>,
        receiver:
                async_channel::Receiver<(
                    String,
                    CreateTaskRequest,
                    TaskStatus,
                    Arc<Mutex<WorkerPool>>,
                )>,
    ) -> WorkerPool {
        for _ in 0..workers_count {
            create_worker(receiver.clone());
        }

        WorkerPool {
            // workers,
            workers_count,
            currently_working_count: 0,
            sender,
            receiver,
        }
    }

    pub fn is_free_worker(&self) -> bool {
        return self.currently_working_count < self.workers_count;
    }

    /// Increases amount of currently working threads and send task in receiver
    /// for free thread to pick up it
    pub async fn do_task(
        &mut self,
        id: &str,
        task: CreateTaskRequest,
        task_status: TaskStatus,
        worker_pool: Arc<Mutex<WorkerPool>>,
    ) {
        self.currently_working_count += 1;
        let _ = self
            .sender
            .send((id.to_string(), task, task_status, worker_pool))
            .await;
    }
}

/// Creates tokio thread that will execute python scripts and binary files
fn create_worker(
    receiver: 
            async_channel::Receiver<(
                String,
                CreateTaskRequest,
                TaskStatus,
                Arc<Mutex<WorkerPool>>,
            )>,
) {
    task::spawn(async move {
        loop {
            if let Ok((id, task, task_status_hashmap, worker_pool)) = receiver.recv().await {
                {
                    let mut task_status = task_status_hashmap.lock().await;
                    let status = task_status.get_mut(&id).unwrap();
                    status.status = "RUNNING".to_string();
                    status.meta.started_at = Some(Utc::now().to_string());
                }

                let (stdout, stderr, execution_result) = match task.task_type.as_str() {
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
            }
        }
    });
}
