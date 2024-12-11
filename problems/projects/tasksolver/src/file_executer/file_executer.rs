use base64::prelude::*;
use std::process::Stdio;
use tokio::process::Command;
use std::fs::File;
use std::io::Write;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn create_temporary_binary_file(decoded_file: &Vec<u8>, id: &str) -> (String, String) {
    let path = format!("{}", id);
    let mut temporary_file = File::create(path.clone()).unwrap();
    let _ = temporary_file.write_all(decoded_file.as_slice());

    let mut permissions = temporary_file.metadata().unwrap().permissions();
    permissions.set_mode(0o777);
    let _ = temporary_file.set_permissions(permissions);

    let execute_path = format!("./{}", id);

    (path, execute_path)
}

pub async fn binary_execute(
    id: String,
    base64_encoded_file: String,
    arguments: String,
) -> (String, Option<String>, String) {
    // let binary_content = fs::read(base64_encoded_file).unwrap();
    // let encoded_file = BASE64_STANDARD.encode(binary_content);

    let decoded_file = BASE64_STANDARD.decode(base64_encoded_file).unwrap();
    // let decoded_file = BASE64_STANDARD.decode(encoded_file).unwrap();

    let (temporary_file_path, execute_path) = create_temporary_binary_file(&decoded_file, &id);

    let output = Command::new(execute_path)
        .arg(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .unwrap();

    let _ = fs::remove_file(temporary_file_path);

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !output.status.success() {
        return (stdout, Some(stderr), "FAILED".to_string());
    }

    (stdout, Some(stderr), "SUCCESS".to_string())
}

pub async fn python_execute(
    python_code: String,
    arguments: String,
) -> (String, Option<String>, String) {
    let output = Command::new("python3")
        .arg("-c")
        .arg(python_code)
        .arg(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !output.status.success() {
        return (stdout, Some(stderr), "FAILED".to_string());
    }

    (stdout, Some(stderr), "SUCCESS".to_string())
}
