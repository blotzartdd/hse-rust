use mmv::from_template_handler::from_template_handler::MatchedFiles;
use mmv::to_template_handler::to_template_handler::FileMover;
use std::process;
use std::path::PathBuf;

fn get_directory_filenames(directory_path: &PathBuf) -> Vec<String> {
    let moved_files = std::fs::read_dir(directory_path.clone()).unwrap();
    let mut moved_filenames = Vec::new();

    for file in moved_files {
        match file {
                Ok(file) => {
                    let filepath = file.path();
                    let filename = filepath.to_str().unwrap().split('/').last().unwrap();
                    if filename != ".DS_Store" {
                        moved_filenames.push(filename.to_owned());
                    }
                }
                Err(_) => process::exit(42)
            }
    }

    moved_filenames
}

#[test]
fn it_works() {
    let mut from_directory_path = std::env::current_dir().unwrap();
    from_directory_path.push("tests/test_data/from_test_folder");
    let mut from_pattern = "some_*_filename.*";

    let mut to_directory_path =  std::env::current_dir().unwrap();
    to_directory_path.push("tests/test_data/to_test_folder");
    let mut to_pattern = "changed_#1_filename.#2";
    let force_flag = false;

    let matched_files = MatchedFiles::new(&from_directory_path, from_pattern);
    let file_mover = FileMover::new(
        &to_directory_path,
        to_pattern,
        force_flag,
    );

    file_mover.move_files_by_pattern(&matched_files);
    let mut moved_filenames = get_directory_filenames(&to_directory_path);

    moved_filenames.sort();
    assert_eq!(moved_filenames.len(), 3);
    assert_eq!(moved_filenames, vec!["changed_A_filename.cpp", "changed_B_filename.rs", "changed_C_filename.bin"]);

    from_pattern = "changed_*_filename.*";
    to_pattern = "some_#1_filename.#2";
    let reverse_matched_files = MatchedFiles::new(&to_directory_path, from_pattern);
    let file_mover = FileMover::new(
        &from_directory_path,
        to_pattern,
        force_flag,
    );

    file_mover.move_files_by_pattern(&reverse_matched_files);
}
