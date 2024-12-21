use clap::Parser;
use tasksolver::input_parser::ServerStartArguments;
use tasksolver::server::server::start_tasksolver_server;

/// Runs tasksolver with arguments from command line
#[tokio::main]
async fn main() {
    let server_start_arguments = ServerStartArguments::parse();
    start_tasksolver_server(
        server_start_arguments.workers_count,
        &server_start_arguments.address,
        server_start_arguments.port,
    )
    .await;
}
