use super::handlers;
use super::models::requests::{CreateTaskRequest, GetStatusRequest};
use super::server::{ServerInfo, TaskStatus};
use crate::worker_pool::worker_pool::WorkerPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{self, Filter};

/// Get data in json from create task request to give to warp method
fn jsonify_create_task(
) -> impl Filter<Extract = (CreateTaskRequest,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

/// Get data in json from get status request to give to warp method
fn jsonify_get_status_task(
) -> impl Filter<Extract = (GetStatusRequest,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

/// Warp /create_task endpoint that calls create_task handler
fn create_task_route(
    worker_pool: Arc<Mutex<WorkerPool>>,
    task_status: TaskStatus,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create_task")
        .and(warp::post())
        .and(jsonify_create_task())
        .and(warp::any().map(move || worker_pool.clone()))
        .and(warp::any().map(move || task_status.clone()))
        .and_then(|task_request, worker_pool, task_status| async move {
            match handlers::create_task(task_request, worker_pool, task_status).await {
                Ok(create_task_response) => Ok(warp::reply::json(&create_task_response)),
                Err(err) => Err(err),
            }
        })
}

/// Warp /get_status endpoint that call get_status handler
fn get_status_route(
    task_status: TaskStatus,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_status")
        .and(warp::get())
        .and(jsonify_get_status_task())
        .and(warp::any().map(move || task_status.clone()))
        .and_then(|get_status_request, task_status| async move {
            match handlers::get_status(get_status_request, task_status).await {
                Ok(get_task_status_response) => Ok(warp::reply::json(&get_task_status_response)),
                Err(err) => Err(err),
            }
        })
}

/// Warp /get_task_count endpoint that calls get_task_count handler
fn get_task_count_route(
    worker_pool: Arc<Mutex<WorkerPool>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_task_count")
        .and(warp::get())
        .and(warp::any().map(move || worker_pool.clone()))
        .and_then(|worker_pool| async move {
            match handlers::get_task_count(worker_pool).await {
                Ok(get_task_count_response) => Ok(warp::reply::json(&get_task_count_response)),
                Err(err) => Err(err),
            }
        })
}

/// Handling all routes and users requests
pub fn routes_handler(
    server_info: ServerInfo,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_task_route(
        server_info.worker_pool.clone(),
        server_info.task_status.clone(),
    )
    .or(get_status_route(server_info.task_status.clone()))
    .or(get_task_count_route(server_info.worker_pool.clone()))
}
