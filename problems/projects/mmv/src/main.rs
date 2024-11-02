use mmv::from_template_handler::from_template_handler::MatchedFiles;
use mmv::input_parser::parser::Arguments;
use mmv::to_template_handler::to_template_handler::FileMover;

fn main() {
    let arguments = Arguments::new();
    let matched_files = MatchedFiles::new(&arguments.from_path, &arguments.from_pattern);
    let file_mover = FileMover::new(
        &arguments.to_path,
        &arguments.to_pattern,
        arguments.force_flag,
    );
    file_mover.move_files_by_pattern(&matched_files);
}
