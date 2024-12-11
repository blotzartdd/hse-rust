pub mod file_executer;
pub mod input_parser;
pub mod server;
pub mod worker_pool;

use clap::Parser;
use crate::input_parser::input_parser::ServerStartArguments;
use crate::server::server::run;

pub async fn start_tasksolver() {
    let server_start_arguments = ServerStartArguments::parse();
    run(
        server_start_arguments.workers_count,
        &server_start_arguments.address,
        server_start_arguments.port,
    )
    .await;
}
