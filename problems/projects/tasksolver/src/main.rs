use clap::Parser;
use tasksolver::input_parser::ServerStartArguments;
use tasksolver::server::server::TaskSolverServer;

/// Runs tasksolver with arguments from command line
#[tokio::main]
async fn main() {
    let server_start_arguments = ServerStartArguments::parse();
    let tasksolver_server = TaskSolverServer::start_tasksolver_server(
        server_start_arguments.workers_count,
        server_start_arguments.address,
        server_start_arguments.port,
    ).await;
    let _ = tasksolver_server.await;
}
