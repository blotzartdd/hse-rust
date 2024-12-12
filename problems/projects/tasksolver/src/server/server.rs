use std::net::SocketAddr;
use std::sync::Arc;

use super::routes::routes_handler;
use super::server_structures::{
    init_task_queue, init_task_status, monitor_queue, TaskQueue, TaskStatus,
};
use crate::worker_pool::worker_pool::WorkerPool;

use tokio::sync::{mpsc, Mutex};
use tokio::task::{self, JoinHandle};
use warp;

/// Struct of server info that contains
/// thread pool with workers, server queue of tasks
/// and status of all tasks.
pub struct ServerInfo {
    pub worker_pool: Arc<Mutex<WorkerPool>>,
    pub task_queue: TaskQueue,
    pub task_status: TaskStatus,
}

impl ServerInfo {
    /// Creates new server info struct
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

/// Runs server on different ip and port. Creates worker pool with given
/// amount of workers. Creates tokio threads to manage the server and task queue in parallel.
/// Await both threads.
pub async fn run(workers_count: usize, ip: &str, port: u16) {
    let socket = SocketAddr::new(ip.parse().unwrap(), port);

    let (task_sender, task_receiver) = mpsc::channel(4096);
    let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
        workers_count,
        task_sender,
        Arc::new(Mutex::new(task_receiver)),
    )));

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

/// Drops two main threads of server
pub fn kill_server(server: JoinHandle<()>, queue_handler: JoinHandle<()>) {
    drop(server);
    drop(queue_handler);
}
