use mmv::from_template_handler::from_template_handler::MatchedFiles;
use mmv::to_template_handler::to_template_handler::FileMover;
use mmv::input_parser::parser::Arguments;

fn main() {
    let arguments = Arguments::new();
    let matched_files = MatchedFiles::new(&arguments.from_path, &arguments.from_pattern);
    let file_mover = FileMover::new(&arguments.to_path, &arguments.to_pattern);
    file_mover.move_files(&matched_files);
}
