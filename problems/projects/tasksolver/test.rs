use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

fn run_python_script(script: &str) -> Result<(String, String), String> {
    let output = Command::new("python3")
        .arg("-c")
        .arg(script)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Ошибка запуска Python: {}", e))?;

    let stdout = BufReader::new(output.stdout.as_slice())
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Ошибка чтения stdout: {}", e))?
        .join("\n");

    let stderr = BufReader::new(output.stderr.as_slice())
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Ошибка чтения stderr: {}", e))?
        .join("\n");

    if !output.status.success() {
        return Err(format!("Python script failed with exit code: {}, stderr: {}", output.status, stderr));
    }

    Ok((stdout, stderr))
}

fn main() {
    let python_code = r#"
import sys

print(1, 2, 3)
print("Hello from Python stdout!")
print("Error message from Python stderr!", file=sys.stderr)
"#;

    match run_python_script(python_code) {
        Ok((stdout, stderr)) => {
            println!("Stdout:\n{}", stdout);
            println!("Stderr:\n{}", stderr);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
