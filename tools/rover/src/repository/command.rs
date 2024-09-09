use std::process::{ExitStatus, Stdio};

use anyhow::{anyhow, bail, Context, Result};
use colored::Colorize;
use itertools::Itertools;

#[derive(Debug)]
pub enum Command {
    ForbidUnsafe,
    ForbidCollections,
    ForbidStd,
    CargoFmt,
    CargoClippy,
    CargoTest,
    CargoTestValidate,
    CargoTestDebug,
    CargoTestDebugValidate,
    CargoMiriTest,
    CargoCompileTestMiniFrunk,
    CargoCompileTestOrm,
    CargoCompileTestSnapshot,
    PythonTest,
}

pub enum CommandStatus {
    Ok,
    ProcessFailed(ExitStatus),
    ChecksFailed,
}

impl CommandStatus {
    pub fn success(&self) -> bool {
        matches!(self, CommandStatus::Ok)
    }
}

impl Command {
    pub fn from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "forbid-unsafe" => Self::ForbidUnsafe,
            "forbid-collections" => Self::ForbidCollections,
            "forbid-std" => Self::ForbidStd,
            "cargo-fmt" => Self::CargoFmt,
            "cargo-clippy" => Self::CargoClippy,
            "cargo-test" => Self::CargoTest,
            "cargo-test-debug" => Self::CargoTestDebug,
            "cargo-miri-test" => Self::CargoMiriTest,
            "cargo-compile-test-mini-frunk" => Self::CargoCompileTestMiniFrunk,
            "cargo-compile-test-orm" => Self::CargoCompileTestOrm,
            "cargo-compile-test-snapshot" => Self::CargoCompileTestSnapshot,
            "python-test" => Self::PythonTest,
            name => bail!("command \"{name}\" is not supported"),
        })
    }

    pub fn needs_nightly_toolchain(&self) -> bool {
        matches!(self, Self::CargoTestValidate | Self::CargoTestDebugValidate)
    }

    pub fn get_shell_line(&self) -> Result<String> {
        Ok(match self {
            Self::ForbidUnsafe => bail!("no shell line for ForbidUnsafe"),
            Self::ForbidCollections => bail!("no shell line for ForbidCollections"),
            Self::ForbidStd => bail!("no shell line for ForbidStd"),
            Self::CargoFmt => "cargo fmt --check".to_string(),
            Self::CargoClippy => "cargo clippy --release -- -D warnings".to_string(),
            Self::CargoTest => "cargo test --release".to_string(),
            Self::CargoTestValidate => {
                "cargo test --release -- -Z unstable-options --format json".to_string()
            }
            Self::CargoTestDebug => "cargo test".to_string(),
            Self::CargoTestDebugValidate => "cargo test -- -Z unstable-options --format json".to_string(),
            Self::CargoMiriTest => "cargo miri test --release".to_string(),
            Self::CargoCompileTestMiniFrunk => bail!("no shell line for CargoCompileTestMiniFrunk"),
            Self::CargoCompileTestOrm => bail!("no shell line for CargoCompileTestOrm"),
            Self::CargoCompileTestSnapshot => bail!("no shell line for CargoCompileTestSnapshot"),
            Self::PythonTest => "python3 test.py".to_string(),
        })
    }

    pub fn cmd_stdout(&self) -> Stdio {
        match self {
            Self::CargoTestValidate | Self::CargoTestDebugValidate => Stdio::piped(),
            _ => Stdio::inherit(),
        }
    }

    pub fn wait(&self, process: &mut std::process::Child) -> Result<CommandStatus> {
        match self {
            Self::CargoTestValidate | Self::CargoTestDebugValidate => {
                let stdout = process
                    .stdout
                    .as_mut()
                    .context("Could not capture standard output")?;

                let not_finished = cargotest::process_test_output(stdout)?;

                let status = process.wait().context("Command was not running")?;
                if status.success() {
                    if !not_finished.is_empty() {
                        let message = not_finished
                            .into_iter()
                            .sorted()
                            .map(|name| format!("  {name}"))
                            .join("\n");
                        println!(
                            "{}: some tests did not finish:\n{}\n",
                            "error".red().bold(),
                            message
                        );
                        return Ok(CommandStatus::ChecksFailed);
                    }
                    Ok(CommandStatus::Ok)
                } else {
                    Ok(CommandStatus::ProcessFailed(status))
                }
            }
            _ => {
                let status = process.wait().context("Command was not running")?;

                if status.success() {
                    Ok(CommandStatus::Ok)
                } else {
                    Ok(CommandStatus::ProcessFailed(status))
                }
            }
        }
    }

    pub fn get_failure_error(&self, _status: CommandStatus) -> anyhow::Error {
        match self {
            Self::CargoFmt => anyhow!(
                "Format your code as suggested above, \
                 or run `{}` to do it automatically",
                "cargo fmt".bold(),
            ),
            _ => anyhow!(
                "Command {} failed, see message above",
                format!("{self:?}").bold()
            ),
        }
    }
}

mod cargotest {
    use std::{collections::HashSet, process::ChildStdout};

    use anyhow::Result;
    use colored::Colorize;

    pub(super) fn print_log_line(log: &serde_json::Value) {
        let format_result = |result| match result {
            "ok" => "ok".green(),
            error @ ("failed" | "timeout") => error.to_uppercase().red(),
            result => result.normal(),
        };
        match (log["type"].as_str(), log["event"].as_str()) {
            (Some("suite"), Some("started")) => {
                println!(
                    "\nrunning {test_count} tests",
                    test_count = log["test_count"].as_i64().unwrap(),
                );
            }
            (Some("suite"), Some(event @ ("ok" | "failed"))) => {
                println!(
                    "\ntest result: {result}. \
                    {passed} passed; \
                    {failed} failed; \
                    {ignored} ignored; \
                    {measured} measured; \
                    {filtered_out} filtered out; \
                    finished in {exec_time:.2}s\n",
                    result = format_result(event),
                    passed = log["passed"].as_i64().unwrap(),
                    failed = log["failed"].as_i64().unwrap(),
                    ignored = log["ignored"].as_i64().unwrap(),
                    measured = log["measured"].as_i64().unwrap(),
                    filtered_out = log["filtered_out"].as_i64().unwrap(),
                    exec_time = log["exec_time"].as_f64().unwrap(),
                );
            }
            (Some("test"), Some(event @ ("ok" | "failed" | "timeout"))) => {
                println!(
                    "test {name} ... {result}",
                    name = log["name"].as_str().unwrap(),
                    result = format_result(event),
                );
            }
            (Some("test"), Some("started")) => {}
            _ => println!("{log}"),
        }
    }

    pub(super) fn print_failures(failures: &Vec<serde_json::Value>) {
        if failures.is_empty() {
            return;
        }

        println!("\nfailures:\n");
        for log in failures {
            match (log["name"].as_str(), log["stdout"].as_str()) {
                (Some(name), Some(stdout)) => {
                    println!("---- {name} stdout ----");
                    print!("{stdout}");
                    if let Some(message) = log["message"].as_str() {
                        println!("note: {message}");
                    }
                    println!();
                }
                _ => println!("{log}"),
            }
        }

        println!("\nfailures:");
        for test_name in failures.into_iter().filter_map(|log| log["name"].as_str()) {
            println!("    {test_name}");
        }
    }

    pub(super) fn process_test_output(stdout: &mut ChildStdout) -> Result<HashSet<String>> {
        let deserializer = serde_json::Deserializer::from_reader(stdout);

        let mut not_finished = HashSet::new();
        let mut failures = vec![];
        for log in deserializer.into_iter::<serde_json::Value>() {
            let log = log?;

            if matches!(
                (log["type"].as_str(), log["event"].as_str()),
                (Some("test"), Some("failed"))
            ) {
                failures.push(log.clone());
            }

            if matches!(
                (log["type"].as_str(), log["event"].as_str()),
                (Some("suite"), Some("ok" | "failed"))
            ) {
                print_failures(&failures);
            }

            if log["type"] == "test" {
                let name = log["name"].as_str().unwrap();
                if log["event"] == "started" {
                    not_finished.insert(name.to_owned());
                } else if log["event"] == "ok" || log["event"] == "failed" {
                    assert!(not_finished.remove(name));
                }
            }

            print_log_line(&log);
        }

        Ok(not_finished)
    }
}
