use tasksolver::server::server::run;
use tasksolver::input_parser::input_parser::ServerStartArguments;
use clap::Parser;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server_start_arguments = ServerStartArguments::parse();
    Ok(run(server_start_arguments.workers_count, &server_start_arguments.address, server_start_arguments.port).await?)
}
