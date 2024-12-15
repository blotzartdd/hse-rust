use std::fmt;
use std::fs;
use std::error::Error;
use base64::prelude::*;
use clap::{Parser, Subcommand, ValueEnum};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use std::env;

#[derive(Parser)]
#[command(name = "Task Solver client")]
#[command(about = "Client for interacting with the Task Solver server", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    #[arg(short, long, default_value = "8080")]
    port: u16,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateTask {
        #[arg(short, long)]
        task_type: TaskType,
        #[arg(short, long)]
        file_path: String,
        #[arg(short, long, default_value = "")]
        args: String,
    },
    GetStatus {
        #[arg(short, long)]
        id: String,
    },
    GetTaskCount,
}

#[derive(ValueEnum, Clone)]
enum TaskType {
    Python,
    Bin,
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TaskType::Bin => write!(f, "bin"),
            TaskType::Python => write!(f, "python"),
        }
    }
}

#[derive(Serialize)]
struct CreateTaskRequest {
    r#type: String,
    file: String,
    args: String,
}

#[derive(Debug, Deserialize)]
struct CreateTaskResponse {
    id: String,
}

#[derive(Serialize)]
struct StatusRequest {
    id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct StatusResponse {
    status: String,
    meta: Meta,
    result: Option<ResultData>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Meta {
    created_at: String,
    started_at: Option<String>,
    finished_at: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ResultData {
    stdout: Option<String>,
    stderr: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TaskCountResponse {
    tasks: u32,
}

fn build_server_url(address: &str, port: u16, endpoint: &str) -> String {
    format!("http://{}:{}/{}", address, port, endpoint)
}

fn create_task(client: &Client, address: &str, port: u16, task_type: &TaskType, file_path: &str, args: &str) -> Result<String, Box<dyn Error>> {
    let url = build_server_url(address, port, "create_task");
    let file_content = match task_type {
        TaskType::Python => {
            fs::read_to_string(file_path)
        }
        TaskType::Bin => {
            let binary_content = fs::read(file_path)?;
            Ok(BASE64_STANDARD.encode(binary_content))
        }
    }?;
    let request = CreateTaskRequest {
        r#type: task_type.to_string(),
        file: file_content.to_string(),
        args: args.to_string(),
    };
    println!("{:?}", client.post(&url).json(&request).send());
    let response = client.post(&url).json(&request).send()?;
    let response_data: CreateTaskResponse = response.json()?;
    Ok(response_data.id)
}

fn get_status(client: &Client, address: &str, port: u16, task_id: &str) -> Result<StatusResponse, Box<dyn Error>> {
    let url = build_server_url(address, port, "get_status");
    let request = StatusRequest {
        id: task_id.to_string(),
    };
    let response = client.get(&url).json(&request).send()?;
    let status_data = response.json()?;
    Ok(status_data)
}

fn get_task_count(client: &Client, address: &str, port: u16) -> Result<u32, Box<dyn Error>> {
    let url = build_server_url(address, port, "get_task_count");
    let response = client.get(&url).send()?;
    let count_data: TaskCountResponse = response.json()?;
    Ok(count_data.tasks)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let cli = Cli::parse();
    let client = Client::new();

    match &cli.command {
        Commands::CreateTask { task_type, file_path, args } => {
            let task_id = create_task(&client, &cli.address, cli.port, task_type, file_path, args).unwrap();
            println!("New task was created with id: {:?}", task_id);
        }
        Commands::GetStatus { id } => {
            let status = get_status(&client, &cli.address, cli.port, id).unwrap();
            println!("Status of the task with id {} is {:?}", id, status);
        }
        Commands::GetTaskCount => {
            let task_count = get_task_count(&client, &cli.address, cli.port).unwrap();
            println!("Number of tasks in the task solver's queue: {}", task_count);
        }
    }
}
