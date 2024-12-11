use std::process::Stdio;
use tokio::process::Command;
use tokio::io::BufReader;
use base64::prelude::*;

pub async fn binary_execute(base64_encoded_file: String, arguments: String) -> (Option<String>, Option<String>, String) {
    let decoded_file = BASE64_STANDARD.decode(base64_encoded_file).unwrap();
    let binary_code = String::from_utf8(decoded_file).unwrap();
    let output = Command::new(binary_code).arg("Hello, world!").output().await.expect("Failed to execute binary file");

    let status = output.status;
    if status.success() {
        (Some(String::from_utf8(output.stdout).unwrap()), Some(String::from_utf8(output.stderr).unwrap()), "SUCCESS".to_string())
    } else {
        (Some(String::from_utf8(output.stdout).unwrap()), Some(String::from_utf8(output.stderr).unwrap()), "FAILED".to_string())
    }
}

pub async fn python_execute(python_code: String, arguments: String) -> (Option<String>, Option<String>, String)  {
    let output = Command::new("python3")
        .arg("-c")
        .arg(python_code)
        .arg(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output().await.unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap(); 
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !output.status.success() {
        return (Some(stdout), Some(stderr), "FAILED".to_string())
    }

    (Some(stdout), Some(stderr), "SUCCESS".to_string())
}
