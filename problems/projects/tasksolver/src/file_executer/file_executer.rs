use base64::prelude::*;
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::fs::File;
use std::io::Write;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn create_temporary_binary_file(decoded_file: &Vec<u8>, id: &str) -> (String, String) {
    let path = format!("{}.bin", id);
    let mut temporary_file = File::create(path.clone()).unwrap();
    let _ = temporary_file.write_all(decoded_file.as_slice());

    let mut permissions = temporary_file.metadata().unwrap().permissions();
    permissions.set_mode(0o777);
    let _ = temporary_file.set_permissions(permissions);

    let execute_path = format!("./{}.bin", id);

    (path, execute_path)
}

/// Execute base64 encoded binary file (by creating temporary file with name of id)
/// and returns stdout, stderr and SUCCESS/ERROR task status
///
/// # Examples
///
/// use tasksolver::file_executer::file_executer::binary_execute;
///
/// let id = "fb85a3a0-7e7f-4a20-8ced-65b3b2475144";
/// let base64_encoded_file = "ZWNobyBIZWxsbywgd29ybGQh"; // -> echo Hello, world!
/// let arguments = "Hello, world!".to_string();
///
/// let (stdout, stderr, task_status) = binary_execute(id, base64_encoded_file, arguments);
///
/// assert_eq!(stdout, "Hello, world!\n");
/// assert_eq!(stderr, None);
/// assert_eq!(task_status, "SUCCESS".to_string());
pub async fn binary_execute(
    id: String,
    base64_encoded_file: String,
    arguments: String,
) -> (String, Option<String>, String) {
    let decoded_file = BASE64_STANDARD.decode(base64_encoded_file).unwrap();
    let (temporary_file_path, execute_path) = create_temporary_binary_file(&decoded_file, &id);

    let output = Command::new(execute_path)
        .arg(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .unwrap();

    let _ = fs::remove_file(temporary_file_path);

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return (stdout, Some(stderr), "FAILED".to_string());
    }

    (stdout, None, "SUCCESS".to_string())
}

/// Execute python script and returns stdout, stderr and SUCCESS/ERROR task status
///
/// # Examples
///
/// use tasksolver::file_executer::file_executer::python_execute;
///
/// let python_code = "print(Hello, world!)";
/// let arguments = "".to_string();
///
/// let (stdout, stderr, task_status) = python_execute(python_code, arguments);
///
/// assert_eq!(stdout, "Hello, world!");
/// assert_eq!(stderr, None);
/// assert_eq!(task_status, "SUCCESS".to_string());
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

    (stdout, None, "SUCCESS".to_string())
}

#[cfg(test)]
mod test_binary_execute {
    use base64::prelude::*;
    use crate::file_executer::file_executer::binary_execute;

    #[tokio::test]
    async fn test_echo() {
        let id = "fb85a3a0-7e7f-4a20-8ced-65b3b2475144".to_string();
        let base64_encoded_file = BASE64_STANDARD.encode("echo Hello, world!");
        let arguments = "".to_string();

        let (stdout, stderr, task_status) = binary_execute(id, base64_encoded_file, arguments).await;
        assert_eq!(stdout, "Hello, world!\n");
        assert_eq!(stderr, None);
        assert_eq!(task_status, "SUCCESS".to_string());
    }

    #[tokio::test]
    async fn test_echo_with_special_symbol() {
        let id = "fb85a3a0-7e7f-4a20-8ced-65b3b2475145".to_string();
        let base64_encoded_file = BASE64_STANDARD.encode("echo Hello,\n world!");
        let arguments = "".to_string();

        let (stdout, stderr, task_status) = binary_execute(id, base64_encoded_file, arguments).await;
        assert_eq!(stdout, "Hello,\n");
        assert_eq!(stderr, Some("./fb85a3a0-7e7f-4a20-8ced-65b3b2475145.bin: line 2: world!: command not found\n".to_string()));
        assert_eq!(task_status, "FAILED".to_string());
    }
}

#[cfg(test)]
mod test_python_execute {
    use crate::file_executer::file_executer::python_execute;

    #[tokio::test]
    async fn test_print() {
        let python_code = "print('Hello, world!')".to_string();
        let arguments = "".to_string();

        let (stdout, stderr, task_status) = python_execute(python_code, arguments).await;
        assert_eq!(stdout, "Hello, world!\n");
        assert_eq!(stderr, None);
        assert_eq!(task_status, "SUCCESS".to_string());
    }

    #[tokio::test]
    async fn test_cycle() {
        let python_code = "for i in range(5):
                            print(i)".to_string();
        let arguments = "".to_string();

        let (stdout, stderr, task_status) = python_execute(python_code, arguments).await;
        assert_eq!(stdout, "0\n1\n2\n3\n4\n");
        assert_eq!(stderr, None);
        assert_eq!(task_status, "SUCCESS".to_string());
    }

    #[tokio::test]
    async fn test_zero_division_error() {
        let python_code = "print(1 / 0)".to_string();
        let arguments = "".to_string();

        let (stdout, stderr, task_status) = python_execute(python_code, arguments).await;
        assert_eq!(stdout, "");
        assert_eq!(stderr, Some("Traceback (most recent call last):\n  File \"<string>\", line 1, in <module>\nZeroDivisionError: division by zero\n".to_string()));
        assert_eq!(task_status, "FAILED".to_string());
    }

    #[tokio::test]
    async fn test_arguments() {
        let python_code = "import sys

print(sys.argv[1])".to_string();
        let arguments = "test_argument".to_string();

        let (stdout, stderr, task_status) = python_execute(python_code, arguments).await;
        assert_eq!(stdout, "test_argument\n");
        assert_eq!(stderr, None);
        assert_eq!(task_status, "SUCCESS".to_string());
    }
}
