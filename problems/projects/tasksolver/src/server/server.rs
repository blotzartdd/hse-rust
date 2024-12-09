use std::net::{IpAddr, SocketAddr};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

use crate::worker_pool::worker_pool::WorkerPool;

pub async fn run(workers_count: u64, ip: &str, port: u16) -> std::io::Result<()> {
    let socket = SocketAddr::new(ip.parse().unwrap(), port);
    let listener = tokio::net::TcpListener::bind(socket).await?;
    let worker_pool = WorkerPool::new(workers_count);

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("New user with addr {:?} connected!", addr); 
            },
            Err(_) => {},
        }
    }
}
