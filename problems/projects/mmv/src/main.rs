use mmv::from_path_handler::from_path_handler::MatchedFiles;
use mmv::input_parser::parser::Arguments;

fn main() {
    let arguments = Arguments::new();
    let matched_files = MatchedFiles::new(&arguments.from_path, &arguments.from_pattern);
    for el in matched_files.file_path_matchings {
        print!("{:?}: ", el.0);
        for matching in el.1 {
            print!("{} ", matching);
        }
        println!("");
    }
}
