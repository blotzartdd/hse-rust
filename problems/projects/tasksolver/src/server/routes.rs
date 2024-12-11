use super::handlers;
use super::models::requests::{CreateTaskRequest, GetStatusRequest};
use super::server::ServerInfo;
use super::server_structures::{TaskQueue, TaskStatus};
use warp::{self, Filter};

fn jsonify_create_task(
) -> impl Filter<Extract = (CreateTaskRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 1024).and(warp::body::json())
}

fn jsonify_get_status_task(
) -> impl Filter<Extract = (GetStatusRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 1024).and(warp::body::json())
}

fn create_task_route(
    task_queue: TaskQueue,
    task_status: TaskStatus,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create_task")
        .and(warp::post())
        .and(jsonify_create_task())
        .and(warp::any().map(move || task_queue.clone()))
        .and(warp::any().map(move || task_status.clone()))
        .and_then(handlers::create_task)
}

fn get_status_route(
    task_status: TaskStatus,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_status")
        .and(warp::get())
        .and(jsonify_get_status_task())
        .and(warp::any().map(move || task_status.clone()))
        .and_then(handlers::get_status)
}

fn get_task_count_route(
    task_queue: TaskQueue,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_task_count")
        .and(warp::get())
        .and(warp::any().map(move || task_queue.clone()))
        .and_then(handlers::get_task_count)
}

pub fn routes_handler(
    server_info: ServerInfo,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_task_route(
        server_info.task_queue.clone(),
        server_info.task_status.clone(),
    )
    .or(get_status_route(server_info.task_status.clone()))
    .or(get_task_count_route(server_info.task_queue.clone()))
}
