use mmv::from_template_handler::from_template_handler::MatchedFiles;
use mmv::to_template_handler::to_template_handler::FileMover;
use std::process;
use std::path::PathBuf;
use mmv::run_mmv_with_arguments;


#[test]
fn it_works() {
    let mut from_directory_path = std::env::current_dir().unwrap();
    from_directory_path.push("tests/test_data/integration_tests_data/it_works/from_test_folder");
    let mut from_pattern = "some_*_filename.*";

    let mut to_directory_path =  std::env::current_dir().unwrap();
    to_directory_path.push("tests/test_data/integration_tests_data/it_works/to_test_folder");
    let mut to_pattern = "changed_#1_filename.#2";

    let force_flag = false;

    run_mmv_with_arguments(&from_directory_path, from_pattern, &to_directory_path, to_pattern, force_flag);

    let mut moved_filenames = get_directory_filenames(&to_directory_path);
    moved_filenames.sort();
    assert_eq!(moved_filenames.len(), 3);
    assert_eq!(moved_filenames, vec!["changed_A_filename.cpp", "changed_B_filename.rs", "changed_C_filename.bin"]);

    from_pattern = "changed_*_filename.*";
    to_pattern = "some_#1_filename.#2";
    move_files_back(&to_directory_path, from_pattern, &from_directory_path, to_pattern);
}

#[test]
fn test_rename_files() {
    let mut from_directory_path = std::env::current_dir().unwrap();
    from_directory_path.push("tests/test_data/integration_tests_data/rename");

    let mut from_pattern = "some_*_filename.*";
    let mut to_pattern = "changed_#1_filename.#2";
    let force_flag = false;

    run_mmv_with_arguments(&from_directory_path, from_pattern, &from_directory_path, to_pattern, force_flag);

    let mut moved_filenames = get_directory_filenames(&from_directory_path);
    moved_filenames.sort();
    assert_eq!(moved_filenames.len(), 3);
    assert_eq!(moved_filenames, vec!["changed_A_filename.cpp", "changed_B_filename.rs", "changed_C_filename.bin"]);
 
    from_pattern = "changed_*_filename.*";
    to_pattern = "some_#1_filename.#2";
    move_files_back(&from_directory_path, from_pattern, &from_directory_path, to_pattern);
}

#[test]
fn test_force_flag() { 
    let mut from_directory_path = std::env::current_dir().unwrap();
    from_directory_path.push("tests/test_data/integration_tests_data/force_flag/from_test_folder");
    let from_pattern = "*";

    let mut to_directory_path = std::env::current_dir().unwrap();
    to_directory_path.push("tests/test_data/integration_tests_data/force_flag/to_test_folder_with_existing_files");
    let to_pattern = "#1";

    let force_flag = true;

    run_mmv_with_arguments(&from_directory_path, from_pattern, &to_directory_path, to_pattern, force_flag);

    let mut moved_filenames = get_directory_filenames(&to_directory_path);
    moved_filenames.sort();
    assert_eq!(moved_filenames.len(), 6);
    assert_eq!(moved_filenames, vec!["MAKE_YOUR_DREAMS_COME_TRUE_with_rust_ofc.rs", "exist.bin", "simple-pattern.c",
        "some_A_filename.cpp", "some_B_filename.rs", "some_C_filename.bin"]);

    move_files_back(&to_directory_path, from_pattern, &from_directory_path, to_pattern);
}

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

fn move_files_back(from_directory_path: &PathBuf, from_pattern: &str, to_directory_path: &PathBuf, to_pattern: &str) {
    run_mmv_with_arguments(from_directory_path, from_pattern, to_directory_path, to_pattern, false); 
}
