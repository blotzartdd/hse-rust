use tokio::task::JoinHandle;
use crate::server::models::requests::CreateTaskRequest;

use tokio::sync::mpsc;

pub struct WorkerPool {
    workers: Vec<Worker>,
    workers_count: usize,
    currently_working_count: usize,
    receiver: mpsc::Receiver<CreateTaskRequest>
}

pub struct Worker {
    worker_thread: JoinHandle<()>,
}

impl WorkerPool {
    pub fn new(workers_count: usize, receiver: mpsc::Receiver<CreateTaskRequest>) -> WorkerPool {
        let mut workers = Vec::new();
        for _ in 0..workers_count {
            let worker = create_worker();
            workers.push(worker);
        }

        WorkerPool {
            workers,
            workers_count,
            currently_working_count: 0,
            receiver,
        }
    }

    // pub fn make_task();
}

async fn output() {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
}

fn create_worker() -> Worker {
    let worker_thread = tokio::spawn(async move {
        loop {
            let _ = output().await;
        }
    });

    Worker {
        worker_thread,
    }
}
