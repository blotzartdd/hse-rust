pub mod from_template_handler;
pub mod input_parser;
pub mod to_template_handler;
pub mod utils;

use crate::input_parser::parser::Arguments;
use crate::from_template_handler::from_template_handler::MatchedFiles;
use crate::to_template_handler::to_template_handler::FileMover;

use std::path::PathBuf;

pub fn run_mmv() {
    let arguments = Arguments::new();
    let matched_files = MatchedFiles::new(&arguments.from_path, &arguments.from_pattern);
    let file_mover = FileMover::new(
        &arguments.to_path,
        &arguments.to_pattern,
        arguments.force_flag,
    );
    file_mover.move_files_by_pattern(&matched_files);
}

pub fn run_mmv_with_arguments(from_path: &PathBuf, from_pattern: &str, to_path: &PathBuf, to_pattern: &str, force_flag: bool) {
    let matched_files = MatchedFiles::new(from_path, &from_pattern);
    let file_mover = FileMover::new(
        to_path,
        to_pattern,
        force_flag,
    );
    file_mover.move_files_by_pattern(&matched_files);
}
