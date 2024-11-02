use crate::from_template_handler::from_template_handler::MatchedFiles;
use crate::utils::utils::{check_folder_existence, is_file_exist};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

pub struct FileMover {
    pub to_path: PathBuf,
    pub to_pattern: String,
    pub force_flag: bool,
}

impl FileMover {
    pub fn new(to_path: &PathBuf, to_pattern: &str, force_flag: bool) -> FileMover {
        FileMover {
            to_path: to_path.clone(),
            to_pattern: to_pattern.to_string(),
            force_flag,
        }
    }

    pub fn move_files(self, matched_files: &MatchedFiles) {
        check_folder_existence(&self.to_path);
        let new_filepath_hashmap = self.get_new_filepath_hashmap(
            &matched_files.filepath_vec,
            &matched_files.filepath_matchings,
        );

        for (filepath, new_filepath) in new_filepath_hashmap.into_iter() {
            match Self::move_file(filepath.to_str().unwrap(), new_filepath.to_str().unwrap()) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!(
                        "mmv: Error {} when try to move {}",
                        err,
                        filepath.to_str().unwrap()
                    );
                    process::exit(42);
                }
            }

            println!(
                "{} -> {}",
                filepath.to_str().unwrap(),
                new_filepath.to_str().unwrap()
            );
        }
    }

    fn move_file(source: &str, destination: &str) -> Result<(), io::Error> {
        fs::rename(source, destination)?;
        Ok(())
    }

    fn replace_markers_with_matchings(pattern: &str, matchings: &Vec<String>) -> String {
        let mut new_filename = pattern.to_string();
        let mut marker_index = 1;
        let mut marker: String = "#".to_owned();
        marker.push_str(&marker_index.to_string());

        while new_filename.find(&marker).is_some() {
            if marker_index > matchings.len() {
                eprintln!("mmv: Marker index is greater than * amount");
                process::exit(42);
            }

            new_filename = new_filename.replace(&marker, &matchings[marker_index - 1]);
            marker_index += 1;
            marker.replace_range(1.., &marker_index.to_string())
        }

        if marker_index <= matchings.len() {
                eprintln!("mmv: Marker indexes were not correctly covered by *");
                process::exit(42);
        }

        new_filename
    }

    fn get_new_filepath_hashmap(
        self,
        filepath_vec: &Vec<PathBuf>,
        filepath_matchings: &HashMap<PathBuf, Vec<String>>,
    ) -> HashMap<PathBuf, PathBuf> {
        let mut new_filepath_hashmap = HashMap::new();
        for filepath in filepath_vec.iter() {
            filepath_matchings
                .get(filepath)
                .into_iter()
                .for_each(|matching_vec| {
                    let new_filename =
                        Self::replace_markers_with_matchings(&self.to_pattern, matching_vec);
                    let new_filepath = self.to_path.join(new_filename.clone());

                    if is_file_exist(&new_filepath) && !self.force_flag {
                        eprintln!(
                            "mmv: Not able to replace existing file: {}",
                            new_filepath.to_str().unwrap()
                        );
                        process::exit(42);
                    }

                    new_filepath_hashmap.insert(filepath.clone(), new_filepath);
                });
        }

        new_filepath_hashmap
    }
}
