use crate::file_executer::file_executer::{binary_execute, python_execute};
use crate::server::models::requests::CreateTaskRequest;
use crate::server::server::TaskStatus;
use chrono::prelude::*;

use tokio::task;

pub struct TaskInfo {
    pub id: String,
    pub task_request: CreateTaskRequest,
    pub task_status: TaskStatus,
}

impl TaskInfo {
    pub fn new(id: String, task_request: CreateTaskRequest, task_status: TaskStatus) -> TaskInfo {
        TaskInfo {
            id,
            task_request,
            task_status,
        }
    }
}

// Struct of tokio threads that will be taking tasks from sender and execute them.
pub struct WorkerPool {
    // Amount of threads in thread pool
    workers_count: usize,
    // Tokio sender that sends task id, task request, task status hashmap and current worker pool
    pub sender: async_channel::Sender<TaskInfo>,
    // Tokio receiver that recieve task id, task request, task status hashmap and current worker pool
    pub receiver: async_channel::Receiver<TaskInfo>,
}

impl WorkerPool {
    /// Creates WorkerPool struct with given amount of workes, sender and receiver.
    pub fn new(
        workers_count: usize,
        sender: async_channel::Sender<TaskInfo>,
        receiver: async_channel::Receiver<TaskInfo>,
    ) -> WorkerPool {
        for _ in 0..workers_count {
            create_worker(receiver.clone());
        }

        WorkerPool {
            workers_count,
            sender,
            receiver,
        }
    }

    /// Increases amount of currently working threads and send task in receiver
    /// for free thread to pick up it
    pub async fn do_task(&mut self, task_info: TaskInfo) {
        let _ = self.sender.send(task_info).await;
    }
}

/// Creates tokio thread that will execute python scripts and binary files
fn create_worker(receiver: async_channel::Receiver<TaskInfo>) {
    task::spawn(async move {
        loop {
            if let Ok(task_info) = receiver.recv().await {
                start_running_task(&task_info.id, task_info.task_status.clone());

                let task_request = task_info.task_request;
                let (stdout, stderr, execution_result) = match task_request.task_type.as_str() {
                    "python" => python_execute(task_request.file, task_request.args).await,
                    "bin" => {
                        binary_execute(task_info.id.clone(), task_request.file, task_request.args)
                            .await
                    }
                    _ => ("".to_string(), None, "".to_string()),
                };

                finish_running_task(
                    &task_info.id,
                    task_info.task_status,
                    stdout,
                    stderr,
                    execution_result,
                );
            }
        }
    });
}

fn start_running_task(id: &str, task_status: TaskStatus) {
    let task_status = task_status.task_status_chashmap;
    let mut status = task_status.get_mut(id).unwrap();
    status.status = "RUNNING".to_string();
    status.meta.started_at = Some(Utc::now().to_string());
}

fn finish_running_task(
    id: &str,
    task_status: TaskStatus,
    stdout: String,
    stderr: Option<String>,
    execution_result: String,
) {
    let task_status = task_status.task_status_chashmap;
    let mut status = task_status.get_mut(id).unwrap();
    status.result.stdout = stdout;
    status.result.stderr = stderr;
    status.status = execution_result;
    status.meta.finished_at = Some(Utc::now().to_string());
}
