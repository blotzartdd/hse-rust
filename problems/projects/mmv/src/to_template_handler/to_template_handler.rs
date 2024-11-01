use std::path::PathBuf;
use crate::from_template_handler::from_template_handler::MatchedFiles;
use crate::utils::utils::check_folder_existence;
use std::fs;
use std::io;

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

        for file in matched_files.file_path_vec.iter() {
            matched_files.file_path_matchings.get(file).into_iter().for_each(|matching_vec| {
                let new_filename = Self::replace_tags_with_matchings(&self.to_pattern, matching_vec);
                let new_filepath = self.to_path.join(new_filename.clone());

                let _ = match Self::move_file(file.to_str().unwrap(), new_filepath.to_str().unwrap()) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        eprintln!("mmv: Error when try to move '{}': {}", file.to_str().unwrap(), err);
                        Err(err)
                    }
                };
                
                println!("{} -> {}", file.to_str().unwrap(), new_filename);
            });
        }
    }

    fn replace_tags_with_matchings(pattern: &str, matchings: &Vec<String>) -> String {
        let mut new_filename = pattern.to_string();
        let mut tag_index = 1;
        let mut tag: String = "#".to_owned();
        tag.push_str(&tag_index.to_string());

        while new_filename.find(&tag).is_some() {
            new_filename = new_filename.replace(&tag, &matchings[tag_index - 1]);
            tag_index += 1;
            tag.replace_range(1.., &tag_index.to_string())
        }

        new_filename
    }

    fn move_file(source: &str, destination: &str) -> Result<(), io::Error> {
        fs::rename(source, destination)?;
        Ok(())
    }
}
