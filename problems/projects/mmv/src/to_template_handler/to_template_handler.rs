use crate::from_template_handler::from_template_handler::MatchedFiles;
use crate::utils::utils::check_folder_existence;
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct FileMover {
    pub to_path: PathBuf,
    pub to_pattern: String,
}

impl FileMover {
    pub fn new(to_path: &PathBuf, to_pattern: &str) -> FileMover {
        FileMover {
            to_path: to_path.clone(),
            to_pattern: to_pattern.to_string(),
        }
    }

    pub fn move_files(self, matched_files: &MatchedFiles) {
        check_folder_existence(&self.to_path);

        for file_path in matched_files.file_path_vec.iter() {
            matched_files
                .file_path_matchings
                .get(file_path)
                .into_iter()
                .for_each(|matching_vec| {
                    let new_filename =
                        Self::replace_markers_with_matchings(&self.to_pattern, matching_vec);
                    let new_filepath = self.to_path.join(new_filename.clone());

                    let _ = match Self::move_file(
                        file_path.to_str().unwrap(),
                        new_filepath.to_str().unwrap(),
                    ) {
                        Ok(_) => Ok(()),
                        Err(err) => {
                            eprintln!(
                                "mmv: Error when try to move '{}': {}",
                                file_path.to_str().unwrap(),
                                err
                            );
                            Err(err)
                        }
                    };

                    println!(
                        "{} -> {}",
                        file_path.to_str().unwrap(),
                        new_filepath.to_str().unwrap()
                    );
                });
        }
    }

    fn replace_markers_with_matchings(pattern: &str, matchings: &Vec<String>) -> String {
        let mut new_filename = pattern.to_string();
        let mut marker_index = 1;
        let mut marker: String = "#".to_owned();
        marker.push_str(&marker_index.to_string());

        while new_filename.find(&marker).is_some() {
            new_filename = new_filename.replace(&marker, &matchings[marker_index - 1]);
            marker_index += 1;
            marker.replace_range(1.., &marker_index.to_string())
        }

        new_filename
    }

    fn move_file(source: &str, destination: &str) -> Result<(), io::Error> {
        fs::rename(source, destination)?;
        Ok(())
    }
}
