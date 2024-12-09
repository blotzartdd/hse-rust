use std::net::SocketAddr;

use crate::worker_pool::worker_pool::WorkerPool;
use super::server_structures::{TaskQueue, init_task_queue, TaskStatus, init_task_status};
use super::models::requests::CreateTaskRequest;
use super::routes::routes_handler;

use warp;
use tokio::sync::mpsc;


pub struct ServerInfo {
    pub worker_pool: WorkerPool,
    pub task_queue: TaskQueue,
    pub task_status: TaskStatus,
    pub task_sender: mpsc::Sender<CreateTaskRequest>,
}

impl ServerInfo {
    pub fn new(worker_pool: WorkerPool, task_queue: TaskQueue,
        task_status: TaskStatus, task_sender: mpsc::Sender<CreateTaskRequest>) -> ServerInfo {
        ServerInfo {
            worker_pool,
            task_queue,
            task_status,
            task_sender,
        }
    }
}

pub async fn run(workers_count: usize, ip: &str, port: u16) {
    let socket = SocketAddr::new(ip.parse().unwrap(), port);

    let (task_sender, task_receiver) = mpsc::channel(4096);
    let worker_pool = WorkerPool::new(workers_count, task_receiver);

    let task_queue = init_task_queue();
    let task_status = init_task_status();

    let server_info = ServerInfo::new(worker_pool, task_queue, task_status, task_sender);

    warp::serve(routes_handler(server_info)).run(socket).await;
}
