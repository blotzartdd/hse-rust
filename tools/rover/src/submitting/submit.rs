use crate::{launch_git, repository::repo::Repository};
use anyhow::{bail, Result};
use colored::Colorize;
use indoc::printdoc;
use std::{path::Path, process};

const MANYTASK_URL: &str = "https://rust-hse-2024.azazkamaz.me";
const SCOREBOARD_URL: &str = "https://rust-hse-2024.azazkamaz.me/scoreboard";
const ACTIONS_URL: &str = "https://rust-hse-2024.azazkamaz.me/repo/actions";

pub fn submit_problem(problem_path: &Path, message: &str) -> Result<()> {
    let repo = Repository::from_path(problem_path)?;
    let problem = repo.problem_from_path(problem_path)?;
    let repo_path = repo.get_path();
    if !launch_git!(repo_path, "add", problem.relative_path()) {
        bail!("git add failed");
    }
    if !launch_git!(repo_path, "commit", "-m", message, "--allow-empty") {
        bail!("git commit failed: either no changes since the last commit or git failed")
    }
    if !launch_git!(
        repo_path,
        "push",
        "--force",
        "origin",
        &format!("HEAD:{}", problem.branch_name())
    ) {
        bail!("git push failed")
    }
    printdoc! {"
        Check results here:   {actions_url}

        Other useful links:
          Course page:        {course_url}
          Scoreboard sheet:   {scoreboard_url}
        ",
        actions_url = ACTIONS_URL.bold(),
        course_url = MANYTASK_URL.bold(),
        scoreboard_url = SCOREBOARD_URL.bold(),
    };
    Ok(())
}
