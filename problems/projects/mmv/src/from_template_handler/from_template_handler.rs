use crate::utils::utils::escape_special_regex_chars;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;

pub struct MatchedFiles {
    pub file_path_vec: Vec<PathBuf>,
    pub file_path_matchings: HashMap<PathBuf, Vec<String>>,
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
            process::exit(42);
        }

        MatchedFiles {
            file_path_vec: matched_files,
            file_path_matchings: matched_files_matchings,
        }
    }

    fn make_correct_pattern(pattern: &str) -> String {
        let mut pattern = (*pattern).to_string();
        pattern = escape_special_regex_chars(&pattern);
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

        (matched_files, matched_files_matchings)
    }
}
