use std::net::SocketAddr;
use std::sync::Arc;

use super::routes::routes_handler;
use super::server_structures::{
    init_task_queue, init_task_status, monitor_queue, TaskQueue, TaskStatus,
};
use crate::worker_pool::worker_pool::WorkerPool;

use tokio::sync::{mpsc, Mutex};
use tokio::task;
use warp;

pub struct ServerInfo {
    pub worker_pool: Arc<Mutex<WorkerPool>>,
    pub task_queue: TaskQueue,
    pub task_status: TaskStatus,
}

impl ServerInfo {
    pub fn new(
        worker_pool: Arc<Mutex<WorkerPool>>,
        task_queue: TaskQueue,
        task_status: TaskStatus,
    ) -> ServerInfo {
        ServerInfo {
            worker_pool,
            task_queue,
            task_status,
        }
    }
}

pub async fn run(workers_count: usize, ip: &str, port: u16) {
    let socket = SocketAddr::new(ip.parse().unwrap(), port);

    let (task_sender, task_receiver) = mpsc::channel(4096);
    let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
        workers_count,
        task_sender,
        Arc::new(Mutex::new(task_receiver)),
    ).await));

    let task_queue = init_task_queue();
    let task_status = init_task_status();

    let server_info = ServerInfo::new(worker_pool.clone(), task_queue.clone(), task_status.clone());

    let server = task::spawn(async move {
        warp::serve(routes_handler(server_info)).run(socket).await;
    });

    let queue_handler = task::spawn(async move {
        monitor_queue(task_queue, task_status, worker_pool).await;
    });

    server.await.unwrap();
    queue_handler.await.unwrap();
}
