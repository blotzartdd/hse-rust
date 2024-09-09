use anyhow::Result;
use clap::{Arg, Command};
use colored::Colorize;
use compose::run_compose::run_compose;
use repository::repo::Repository;
use std::io::Write;
use std::{
    io,
    path::PathBuf,
    process::{ExitCode, Termination},
};
use submitting::submit::submit_problem;
use testing::{report::ReportType, test::test_problem};

#[cfg(feature = "check-version")]
use util::check_version::ensure_version_is_latest;

mod compose;
mod repository;
mod submitting;
mod testing;
mod util;

struct NiceMainResult(Result<()>);

impl From<Result<()>> for NiceMainResult {
    fn from(res: Result<()>) -> Self {
        Self(res)
    }
}

impl Termination for NiceMainResult {
    fn report(self) -> std::process::ExitCode {
        match self.0 {
            Ok(val) => val.report(),
            Err(err) => {
                // Ignore error if the write fails, for example because stderr is
                // already closed. There is not much point panicking at this point.
                let _ = writeln!(io::stderr(), "{}: {err:?}", "Error".red().bold());
                ExitCode::FAILURE
            }
        }
    }
}

fn run_rover() -> Result<()> {
    let matches = Command::new("rover")
        .about("Helper tool for the Rust language course")
        .subcommand(
            Command::new("submit")
                .about("Submit the results to the CI")
                .arg(
                    Arg::new("path")
                        .long("path")
                        .help("Path to problem to submit")
                        .required(false)
                        .default_value(".")
                        .hide_default_value(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("message")
                        .long("message")
                        .help("Commit message in CI")
                        .required(false)
                        .default_value("Automatic message by rover-submit")
                        .takes_value(true)
                )
        )
        .subcommand(
            Command::new("test")
                .about("Test the problem using testing configuration file")
                .arg(
                    Arg::new("path")
                        .long("path")
                        .help("Path to directory with a \".config.yml\" of the problem within course repository")
                        .required(false)
                        .default_value(".")
                        .hide_default_value(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("move-files")
                        .long("move-files")
                        .help("Path to the repository with solutions to move files from")
                        .required(false)
                        .requires("path")
                        .takes_value(true)
                )
                .arg(
                    Arg::new("checkout-branch")
                        .long("checkout-branch")
                        .help("Do we need to checkout branch with problem or not")
                        .required(false)
                        .requires("move-files")
                        .takes_value(false)
                )
                .arg(
                    Arg::new("report-to")
                        .long("report-to")
                        .help("Set system that will accept the results of testing")
                        .required(false)
                        .default_value("no-report")
                        .takes_value(true)
                )
        )
        .subcommand(
            Command::new("compose")
                .about("Make public repository from private")
                .arg(
                    Arg::new("input")
                        .long("input")
                        .help("Path to the private repository with \".compose.yml\" file")
                        .required(false)
                        .default_value(".")
                        .hide_default_value(true)
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .help("Directory where output will be stored")
                        .required(true)
                        .takes_value(true)
                )
        )
        .arg_required_else_help(true)
        .get_matches();

    match matches.subcommand() {
        Some(("submit", submit_matches)) => {
            let path: PathBuf = submit_matches.value_of("path").unwrap().into();

            #[cfg(feature = "check-version")]
            ensure_version_is_latest(&path)?;

            let message = submit_matches.value_of("message").unwrap();
            submit_problem(&path, message)
        }
        Some(("test", test_matches)) => {
            let path: PathBuf = test_matches.value_of("path").unwrap().into();

            #[cfg(feature = "check-version")]
            ensure_version_is_latest(&path)?;

            let repository = Repository::from_path(&path)?;
            let problem = repository.problem_from_path(&path)?;
            let report = ReportType::from_name(test_matches.value_of("report-to").unwrap())?;
            if let Some(solutions_repo) = test_matches.value_of("move-files") {
                let checkout_branch = test_matches.value_of("checkout-branch").is_some();
                let solutions_repo: PathBuf = solutions_repo.into();
                problem.move_solution_files_from(&solutions_repo, checkout_branch)?;
            }
            // TODO: Make testing errors more clear
            let testing_result = test_problem(problem);
            let report_push = report.push_report(testing_result.is_err());
            testing_result.and(report_push)
        }
        Some(("compose", compose_matches)) => {
            let input: PathBuf = compose_matches.value_of("input").unwrap().into();
            let output: PathBuf = compose_matches.value_of("output").unwrap().into();
            run_compose(&input, &output)
        }
        _ => unreachable!(),
    }
}

fn main() -> NiceMainResult {
    run_rover().into()
}
