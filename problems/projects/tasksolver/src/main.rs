use tasksolver::server::server::run;
use tasksolver::input_parser::input_parser::ServerStartArguments;
use clap::Parser;

#[tokio::main]
async fn main() {
    let server_start_arguments = ServerStartArguments::parse();
    run(server_start_arguments.workers_count, &server_start_arguments.address, server_start_arguments.port).await;
}
