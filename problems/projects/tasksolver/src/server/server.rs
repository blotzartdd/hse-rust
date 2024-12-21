use chashmap::CHashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use super::models::responses::GetStatusResponse;
use super::routes::routes_handler;
use crate::worker_pool::worker_pool::WorkerPool;

use tokio::sync::Mutex;
use tokio::task::{self, JoinHandle};
use warp;

/// Task status hashmap for all tasks on server
#[derive(Clone)]
pub struct TaskStatus {
    pub task_status_chashmap: Arc<CHashMap<String, GetStatusResponse>>,
}

impl TaskStatus {
    /// Create task status struct
    pub fn new() -> TaskStatus {
        TaskStatus {
            task_status_chashmap: Arc::new(CHashMap::new()),
        }
    }
}

/// Struct of server info that contains
/// thread pool with workers, server queue of tasks
/// and status of all tasks.
pub struct ServerInfo {
    pub worker_pool: Arc<Mutex<WorkerPool>>,
    pub task_status: TaskStatus,
}

impl ServerInfo {
    /// Creates new server info struct
    pub fn new(worker_pool: Arc<Mutex<WorkerPool>>, task_status: TaskStatus) -> ServerInfo {
        ServerInfo {
            worker_pool,
            task_status,
        }
    }
}

/// Runs server on different ip and port. Creates worker pool with given
/// amount of workers. Creates tokio threads to manage the server and task queue in parallel.
/// Await both threads.
pub async fn start_tasksolver_server(workers_count: usize, ip: &str, port: u16) {
    let socket = SocketAddr::new(ip.parse().unwrap(), port);

    let (task_sender, task_receiver) = async_channel::unbounded();
    let worker_pool = Arc::new(Mutex::new(WorkerPool::new(
        workers_count,
        task_sender,
        task_receiver,
    )));

    let task_status = TaskStatus::new();
    let server_info = ServerInfo::new(worker_pool.clone(), task_status);

    let server = task::spawn(async move {
        warp::serve(routes_handler(server_info)).run(socket).await;
    });

    let _ = server.await;
}

/// Drops two main threads of server
pub fn kill_server(server: JoinHandle<()>, queue_handler: JoinHandle<()>) {
    drop(server);
    drop(queue_handler);
}
