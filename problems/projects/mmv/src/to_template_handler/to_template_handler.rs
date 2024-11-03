use crate::from_template_handler::from_template_handler::MatchedFiles;
use crate::utils::utils::{is_file_exist, is_folder_exist};
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

    pub fn move_files_by_pattern(self, matched_files: &MatchedFiles) {
        if !is_folder_exist(&self.to_path) {
            eprintln!(
                "mmv: Folder '{}' does not exist",
                self.to_path.to_str().unwrap()
            );
            panic!("TO FOLDER NOT EXIST");
            process::exit(42);
        }

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

    pub fn replace_markers_with_matchings(pattern: &str, matchings: &Vec<String>) -> String {
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
            let mut new_filename = self.to_pattern.clone();
            if let Some(matching_vec) = filepath_matchings.get(filepath) {
                new_filename = Self::replace_markers_with_matchings(&self.to_pattern, matching_vec);
            }

            let new_filepath = self.to_path.join(new_filename);

            if is_file_exist(&new_filepath) && !self.force_flag {
                eprintln!(
                    "mmv: Not able to replace existing file: {}",
                    new_filepath.to_str().unwrap()
                );
                panic!("FORCE FLAG");
                process::exit(42);
            }

            new_filepath_hashmap.insert(filepath.clone(), new_filepath);
        }

        new_filepath_hashmap
    }
}

#[cfg(test)]
mod test_file_mover_marker_replace {
    use super::FileMover;

    #[test] 
    fn test_marker_replace1() {
        let pattern = "changed_#1_filename.#2";
        let matchings = vec!["A".to_string(), "cpp".to_string()];

        let replace_result = FileMover::replace_markers_with_matchings(pattern, &matchings);
        assert_eq!(replace_result, "changed_A_filename.cpp");
    }

    #[test]
    fn test_marker_replace2() {
        let pattern = "#1#2file_with_#3a#4_lot_markers#5.#6";
        let matchings = vec!["REALLY".to_string(), "_".to_string(), "A".to_string(),
            "LOT".to_string(), "OF_MARKERS".to_string(), "rs".to_string()];

        let replace_result = FileMover::replace_markers_with_matchings(pattern, &matchings);
        assert_eq!(replace_result, "REALLY_file_with_AaLOT_lot_markersOF_MARKERS.rs");
    }

    #[test]
    fn test_marker_replace3() {
        let pattern = "#3absolutely#1_useful#2_pattern.jpg";
        let matchings = vec!["NO".to_string(), "joke".to_string(), "bimbimbambam".to_string()];

        let replace_result = FileMover::replace_markers_with_matchings(pattern, &matchings);
        assert_eq!(replace_result, "bimbimbambamabsolutelyNO_usefuljoke_pattern.jpg");
    }
}
