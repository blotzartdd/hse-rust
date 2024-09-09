use super::{cargo_root::cargo_root, process::process, prune::prune, skip::skip};
use crate::repository::repo::Repository;
use anyhow::{Context, Result};
use std::path::Path;

pub fn run_compose(input: &Path, output: &Path) -> Result<()> {
    let repository = Repository::from_path(input)?;
    let config = repository.compose_config()?;
    let input = repository.get_path().to_path_buf();
    let output = output
        .canonicalize()
        .context("could not canonicalize output path")?;
    prune(&output, &config)?;
    process(&input, &output, &config)?;
    cargo_root(&output, &config)?;
    skip(&output, &config)
}
