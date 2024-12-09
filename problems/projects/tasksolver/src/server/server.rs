use std::net::SocketAddr;
use std::collections::HashMap;
use crate::worker_pool::worker_pool::WorkerPool;
use super::server_structures::{TaskQueue, init_task_queue, TaskStatus};
use super::routes::routes_handler;
use warp;


pub struct ServerInfo {
    pub worker_pool: WorkerPool,
    pub task_queue: TaskQueue,
    pub task_status: TaskStatus,
}

impl ServerInfo {
    pub fn new(worker_pool: WorkerPool, task_queue: TaskQueue, task_status: TaskStatus) -> ServerInfo {
        ServerInfo {
            worker_pool,
            task_queue,
            task_status,
        }
    }
}

pub async fn run(workers_count: usize, ip: &str, port: u16) {
    let socket = SocketAddr::new(ip.parse().unwrap(), port);
    let worker_pool = WorkerPool::new(workers_count);
    let task_queue = init_task_queue();
    let task_status = HashMap::new();

    let server_info = ServerInfo::new(worker_pool, task_queue, task_status);

    warp::serve(routes_handler(server_info)).run(socket).await;
}
