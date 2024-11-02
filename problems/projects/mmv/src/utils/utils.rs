use std::path::Path;
use std::process;

pub fn escape_special_regex_chars(pattern: &str) -> String {
    let special_chars = r"[]{}()|^$\?+";

    let mut new_pattern = String::new();
    for ch in pattern.chars() {
        if special_chars.contains(ch) && ch != '*' {
            new_pattern.push('\\');
        }
        new_pattern.push(ch);
    }

    new_pattern
}

pub fn check_folder_existence(folder_path: &Path) {
    let folder_exist_result = folder_path.try_exists();
    let is_folder_exist = match folder_exist_result {
        Ok(result) => result,
        Err(_) => {
            eprintln!(
                "mmv: Unable to check folder existence of '{}'",
                folder_path.to_str().unwrap()
            );
            process::exit(42);
        }
    };

    if !is_folder_exist {
        eprintln!(
            "mmv: Folder '{}' does not exist",
            folder_path.to_str().unwrap()
        );
        process::exit(42);
    }
}

pub fn is_file_exist(filepath: &Path) -> bool {
    let file_exist_result = filepath.try_exists();
    match file_exist_result {
        Ok(result) => return result,
        Err(_) => {
            eprintln!(
                "mmv: Unable to check file existence of '{}'",
                filepath.to_str().unwrap()
            );
            process::exit(42);
        }
    };
}
