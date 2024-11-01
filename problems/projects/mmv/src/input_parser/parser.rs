use clap::Parser;
use std::path::{Path, PathBuf};

use crate::utils::utils::check_folder_existence;

#[derive(Parser, Debug)]
struct ArgumentsParse {
    from_template: String,
    to_template: String,
}

pub struct Arguments {
    pub from_path: PathBuf,
    pub from_pattern: String,
    pub to_path: PathBuf,
    pub to_pattern: String,
}

impl Arguments {
    pub fn new() -> Arguments {
        let parsed_arguments: ArgumentsParse = ArgumentsParse::parse();
        let from = Path::new(&parsed_arguments.from_template);
        let to = Path::new(&parsed_arguments.to_template);

        let from_template_folder = from.parent();
        let to_template_folder = to.parent();

        if from_template_folder.is_some() && from_template_folder.unwrap().to_str() != Some("") {
            check_folder_existence(from_template_folder.unwrap());
        }

        Arguments {
            from_path: from_template_folder.unwrap().into(),
            from_pattern: from
                .to_str()
                .unwrap()
                .to_string()
                .split('/')
                .last()
                .unwrap()
                .to_string(),
            to_path: to_template_folder.unwrap().into(),
            to_pattern: to
                .to_str()
                .unwrap()
                .to_string()
                .split('/')
                .last()
                .unwrap()
                .to_string(),
        }
    }
}
