use crate::utils::utils::escape_special_regex_characters;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;

#[derive(Debug)]
pub struct MatchedFiles {
    pub filepath_vec: Vec<PathBuf>,
    pub filepath_matchings: HashMap<PathBuf, Vec<String>>,
}

impl MatchedFiles {
    pub fn new(from_path: &PathBuf, from_pattern: &str) -> MatchedFiles {
        let pattern = Self::make_correct_pattern(from_pattern);
        let regex_pattern = Regex::new(&pattern).unwrap();

        let matching_count = pattern
            .as_bytes()
            .windows(1)
            .filter(|&w| w == "*".as_bytes())
            .count();

        let (matched_files, matched_files_matchings) =
            Self::get_matched_files_info(from_path, regex_pattern, matching_count);

        if matched_files.is_empty() {
            eprintln!(
                "mmv: Files for pattern '{}' not found",
                from_path.join(from_pattern).to_str().unwrap()
            );
            panic!("FILES FOR PATTERN NOT FOUND");
            process::exit(42);
        }

        MatchedFiles {
            filepath_vec: matched_files,
            filepath_matchings: matched_files_matchings,
        }
    }

    fn make_correct_pattern(pattern: &str) -> String {
        let mut pattern = (*pattern).to_string();
        pattern = escape_special_regex_characters(&pattern);
        pattern = pattern.replace("*", "(.*)");

        pattern
    }

    fn get_matched_files_info(
        from_path: &PathBuf,
        regex_pattern: Regex,
        matching_count: usize,
    ) -> (Vec<PathBuf>, HashMap<PathBuf, Vec<String>>) {
        let mut matched_files: Vec<PathBuf> = Vec::new();
        let mut matched_files_matchings: HashMap<PathBuf, Vec<String>> = HashMap::new();

        let files = std::fs::read_dir(from_path).unwrap();
        for file in files {
            match file {
                Ok(file) => {
                    let filepath = file.path();
                    let filename = filepath.to_str().unwrap().split('/').last().unwrap();

                    let matchings_option = regex_pattern.captures(filename);
                    if matchings_option.is_none() {
                        continue;
                    }

                    let matchings = matchings_option.unwrap();
                    if matchings[0] == *filename {
                        let matched_filepath = from_path.join(filename);
                        matched_files.push(matched_filepath.clone());

                        for i in 1..matching_count + 1 {
                            match matched_files_matchings.get_mut(&matched_filepath) {
                                Some(current_matchings) => {
                                    current_matchings.push(matchings[i].to_string());
                                }
                                None => {
                                    matched_files_matchings.insert(
                                        matched_filepath.clone(),
                                        vec![matchings[i].to_string()],
                                    );
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    eprintln!("mmv: File error: {:?}", err);
                    process::exit(42);
                }
            }
        }

        matched_files.sort();

        (matched_files, matched_files_matchings)
    }
}


#[cfg(test)]
mod test_matched_files {
    use crate::from_template_handler::from_template_handler::MatchedFiles;
    use std::path::PathBuf;

   #[test]
   fn test_pattern_without_special_symbols() {
        let mut directory_path = std::env::current_dir().unwrap();
        directory_path.push("tests/test_data/unit_tests_data");
        let pattern = "simple-pattern.c";
        let mut target_filepath = directory_path.clone();
        target_filepath.push(pattern);

        let matched_files = MatchedFiles::new(&directory_path.to_path_buf(), pattern);

        assert_eq!(matched_files.filepath_vec.len(), 1);
        assert_eq!(matched_files.filepath_matchings.len(), 0);
        assert_eq!(matched_files.filepath_vec[0], target_filepath);
    } 

    #[test]
    fn test_all_files_pattern() {
        let mut directory_path = std::env::current_dir().unwrap();
        directory_path.push("tests/test_data/unit_tests_data");
        let pattern = "*";
        let matched_files = MatchedFiles::new(&directory_path.to_path_buf(), pattern);

        assert_eq!(matched_files.filepath_vec.len(), 6);

        let filenames = get_filenames_from_paths(&matched_files.filepath_vec);
        assert_eq!(filenames, vec!["MAKE_YOUR_DREAMS_COME_TRUE_with_rust_ofc.rs", "exist.bin",  "simple-pattern.c",
            "some_A_filename.cpp", "some_B_filename.rs", "some_C_filename.bin"]);
    }

    #[test]
    fn test_pattern1() {
        let mut directory_path = std::env::current_dir().unwrap();
        directory_path.push("tests/test_data/unit_tests_data");
        let pattern = "some_*_filename.*";
        let matched_files = MatchedFiles::new(&directory_path.to_path_buf(), pattern);

        assert_eq!(matched_files.filepath_vec.len(), 3);

        let filenames = get_filenames_from_paths(&matched_files.filepath_vec);
        assert_eq!(filenames, vec!["some_A_filename.cpp", "some_B_filename.rs", "some_C_filename.bin"]);
    }

    #[test]
    fn test_pattern2() {
        let mut directory_path = std::env::current_dir().unwrap();
        directory_path.push("tests/test_data/unit_tests_data");
        let pattern = "s*";
        let matched_files = MatchedFiles::new(&directory_path.to_path_buf(), pattern);

        assert_eq!(matched_files.filepath_vec.len(), 4);

        let filenames = get_filenames_from_paths(&matched_files.filepath_vec);
        assert_eq!(filenames, vec!["simple-pattern.c", "some_A_filename.cpp", "some_B_filename.rs", "some_C_filename.bin"]);
    }

    #[test]
    fn test_pattern_matchings() {
        let mut directory_path = std::env::current_dir().unwrap();
        directory_path.push("tests/test_data/unit_tests_data");
        let pattern = "some_*_filename.*";
        let matched_files = MatchedFiles::new(&directory_path.to_path_buf(), pattern);

        assert_eq!(matched_files.filepath_matchings.len(), 3);

        let mut first_matchings = Vec::new();
        let mut second_matchings = Vec::new();

        for (_, matchings) in matched_files.filepath_matchings.iter() {
            first_matchings.push(matchings[0].clone());
            second_matchings.push(matchings[1].clone());
        }

        first_matchings.sort();
        second_matchings.sort();

        assert_eq!(first_matchings, vec!["A", "B", "C"]);
        assert_eq!(second_matchings, vec!["bin", "cpp", "rs"]);
    }

    fn get_filenames_from_paths(filepath_vec: &Vec<PathBuf>) -> Vec<&str> {
        let mut filenames = Vec::new();
        for filepath in filepath_vec {
            filenames.push(filepath.to_str().unwrap().split('/').last().unwrap());
        }
        
        filenames.sort();
        filenames
    }
}
