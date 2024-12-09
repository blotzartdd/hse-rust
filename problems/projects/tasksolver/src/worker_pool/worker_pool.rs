use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::task::JoinHandle;
use tokio::sync::mpsc::{self, Sender, Receiver};

use tokio::net::TcpStream;

pub struct WorkerPool {
    workers: Vec<Worker>,
    workers_count: usize,
    currently_working_count: usize,
}

pub struct Worker {
    worker_thread: JoinHandle<()>,
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl WorkerPool {
    pub fn new(workers_count: usize) -> WorkerPool {
        let mut workers = Vec::new();
        for i in 0..workers_count {
            let worker = create_worker(i);
            workers.push(worker);
        }

        WorkerPool {
            workers,
            workers_count,
            currently_working_count: 0,
        }
    }

    // pub fn make_task();
}

async fn output() {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
}

fn create_worker(worker_number: usize) -> Worker {
    let worker_thread = tokio::spawn(async move {
        loop {
            let _ = output().await;
            println!("Worker num {} prints!", worker_number);
        }
    });

    let (sender, receiver) = mpsc::channel(worker_number);
    Worker {
        worker_thread,
        sender,
        receiver
    }
}
