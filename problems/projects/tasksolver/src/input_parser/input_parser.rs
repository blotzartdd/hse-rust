use clap::Parser;

#[derive(Parser, Debug)]
pub struct ServerStartArguments {
    #[arg(short = 'w', long = "workers", default_value_t = 1)]
    pub workers_count: usize,
    #[arg(short = 'a', long = "address", default_value = "127.0.0.1")]
    pub address: String,
    #[arg(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,
}
