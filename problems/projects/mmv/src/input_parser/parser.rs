use clap::Parser;
use std::path::{Path, PathBuf};
use std::process;

use crate::utils::utils::is_folder_exist;

#[derive(Parser, Debug)]
/// PARSEEER
struct ArgumentsParse {
    /// Force flag
    #[arg(short = 'f', long = "force", default_value_t = false)]
    force_flag: bool,
    /// Text for from_template
    from_template: String,
    /// Text for to_template
    to_template: String,
}

/// Struct with all parsed arguments that are need for mmv work
#[derive(Debug)]
pub struct Arguments {
    /// Path to the directory from which files to move are taken
    pub from_path: PathBuf,
    /// Pattern by which files are selected in from_path
    pub from_pattern: String,
    /// Path to the directory where mmv will move the files
    pub to_path: PathBuf,
    /// Pattern by which files change their names in the to_path folder
    pub to_pattern: String,
    /// Set force flag
    pub force_flag: bool,
}

impl Arguments {
    /// Creates Arguments struct from console input
    ///
    /// # Examples
    ///
    /// 
    /// ```
    /// // Console input: 'path/to/some_*_filename.*' 'path2/to/changed_#1_filename.#2'
    /// use doc::Arguments;
    /// let arguments = Arguments::new();
    /// assert_eq!(arguments, Arguments {
    ///     from_path: PathBuf("path/to"),
    ///     from_pattern: "some_*_filename.*",
    ///     to_path: PathBuf("path2/to"),
    ///     to_pattern: "changed_#1_filename.#2",
    ///     force_flag: false,
    /// })
    ///
    /// ```
    ///
    /// ```
    /// // Console input: -f 'path/to/simple_pattern.rs' 'path2/to/not_simple_pattern.rs'
    /// use doc::Arguments;
    /// let arguments = Arguments::new();
    /// assert_eq!(arguments, Arguments {
    ///     from_path: PathBuf("path/to"),
    ///     from_pattern: "simple_pattern.rs",
    ///     to_path: PathBuf("path2/to"),
    ///     to_pattern: "not_simple_pattern.rs",
    ///     force_flag: true,
    /// })
    ///
    /// ```
    pub fn new() -> Arguments {
        let parsed_arguments: ArgumentsParse = ArgumentsParse::parse();
        let from_template = Path::new(&parsed_arguments.from_template);
        let to_template = Path::new(&parsed_arguments.to_template);

        let from_template_folder = from_template.parent();
        let to_template_folder = to_template.parent();

        if from_template_folder.is_some() && from_template_folder.unwrap().to_str() != Some("") {
            if !is_folder_exist(from_template_folder.unwrap()) {
                eprintln!(
                    "mmv: Folder '{}' does not exist",
                    from_template_folder.unwrap().to_str().unwrap()
                );
                process::exit(42);
            }
        }

        Arguments {
            from_path: from_template_folder.unwrap().into(),
            from_pattern: from_template
                .to_str()
                .unwrap()
                .to_string()
                .split('/')
                .last()
                .unwrap()
                .to_string(),
            to_path: to_template_folder.unwrap().into(),
            to_pattern: to_template
                .to_str()
                .unwrap()
                .to_string()
                .split('/')
                .last()
                .unwrap()
                .to_string(),
            force_flag: parsed_arguments.force_flag,
        }
    }
}
