use std::path::Path;
use std::process;

/// Escapes special characters that are commonly used in regular expressions
/// 
/// # Examples 
///
/// ```
/// use utils::escape_special_regular_expression_characters;
///
/// let regular_expression = "$print^pretty+string";
/// let result = escape_special_regular_expression_characters(regular_expression);
///
/// assert_eq(result, "\\$print\\^pretty\\+string");
/// ```
pub fn escape_special_regular_expression_characters(pattern: &str) -> String {
    let special_chars = r"[]{}()|^$\?+.";

    let mut new_pattern = String::new();
    for ch in pattern.chars() {
        if special_chars.contains(ch) && ch != '*' {
            new_pattern.push('\\');
        }

        if ch != '\\' {
            new_pattern.push(ch);
        }
    }

    new_pattern
}

/// Safe check of folder existence with special mmv error message
/// 
/// # Examples 
///
/// ```
/// use utils::is_folder_exist;
/// 
/// let folder_path = Path::new("path/to/existing_folder");
/// let result = is_folder_exist(folder_path);
///
/// assert_eq(result, true);
/// ```
///
/// ```
/// use utils::is_folder_exist;
///
/// let folder_path = Path::new("path/to/not_existing_folder");
/// let result = is_folder_exist(folder_path);
///
/// assert_eq(result, false);
/// ```
///
/// ```
/// use utils::is_folder_exist;
///
/// let folder_path = Path::new("path/to/system_folder");
/// let result = is_folder_exist(folder_path); // -> Print error message and exit with non-zero
/// exit code
/// ```
pub fn is_folder_exist(folder_path: &Path) -> bool {
    let folder_exist_result = folder_path.try_exists();
    match folder_exist_result {
        Ok(result) => return result,
        Err(_) => {
            eprintln!(
                "mmv: Unable to check folder existence of '{}'",
                folder_path.to_str().unwrap()
            );
            process::exit(42);
        }
    }
}

/// Safe check of file existence with special mmv error message
/// 
/// # Examples 
///
/// ```
/// use utils::is_file_exist;
///
/// let file_path = Path::new("path/to/existing_file");
/// let result = is_folder_exist(folder_path);
///
/// assert_eq(result, true);
/// ```
///
/// ```
/// use utils::is_file_exist;
///
/// let file_path = Path::new("path/to/not_existing_file");
/// let result = is_folder_exist(folder_path);
///
/// assert_eq(result, false);
/// ```
///
/// ```
/// use utils::is_file_exist;
///
/// let file_path = Path::new("path/to/system_file");
/// let result = is_folder_exist(folder_path); // -> Print error message and exit with non-zero
/// exit code
/// ```
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

#[cfg(test)]
mod test_escape_special_regular_expression_characterss {
    use super::escape_special_regular_expression_characters;

    #[test]
    fn no_special_characters() {
        let mut test_string = "abracadabra";
        assert_eq!(escape_special_regular_expression_characters(test_string), test_string);

        test_string = "bimbimbambam bambambimbim";
        assert_eq!(escape_special_regular_expression_characters(test_string), test_string);
    }

    #[test]
    fn not_ascii_characters() {
        let mut test_string = "русский язык";
        assert_eq!(escape_special_regular_expression_characters(test_string), test_string);

        test_string = "𡨸漢漢字漢字";
        assert_eq!(escape_special_regular_expression_characters(test_string), test_string);
    }

    #[test]
    fn simple_changes() {
        let mut test_string = "[abcd]";
        assert_eq!(escape_special_regular_expression_characters(test_string), "\\[abcd\\]");

        test_string = "$print^pretty+string";
        assert_eq!(
            escape_special_regular_expression_characters(test_string),
            "\\$print\\^pretty\\+string"
        );
    }

    #[test]
    fn all_symbols() {
        let test_string = r"[]{}()|^$\?+.";
        assert_eq!(
            escape_special_regular_expression_characters(test_string),
            "\\[\\]\\{\\}\\(\\)\\|\\^\\$\\\\?\\+\\."
        );
    }

    #[test]
    fn regular_expression() {
        let test_string = r"2020-03-12T13:34:56\.123Z INFO\[org\.example\.Class\]";
        assert_eq!(escape_special_regular_expression_characters(test_string),
            "2020-03-12T13:34:56\\\\.123Z INFO\\\\[org\\\\.example\\\\.Class\\\\]")
    }
}

#[cfg(test)]
mod test_is_file_exist {
    use super::is_file_exist;

    #[test]
    fn test_file_exist() {
        let mut filepath = std::env::current_dir().unwrap();
        filepath.push("tests/test_data/unit_tests_data/exist.bin");
        assert_eq!(is_file_exist(&filepath), true); 
    }

    #[test]
    fn test_file_not_exist() {
        let mut filepath = std::env::current_dir().unwrap();
        filepath.push("tests/test_data/unit_tests_data/not_exist.bin");
        assert_eq!(is_file_exist(&filepath), false); 
    }
}

#[cfg(test)]
mod test_is_folder_exist {
    use super::is_folder_exist;

    #[test]
    fn test_folder_exist() {
        let mut filepath = std::env::current_dir().unwrap();
        filepath.push("tests/test_data/unit_tests_data");
        assert_eq!(is_folder_exist(&filepath), true); 
    }

    #[test]
    fn test_folder_not_exist() {
        let mut filepath = std::env::current_dir().unwrap();
        filepath.push("tests/test_data/some_unknown_folder");
        assert_eq!(is_folder_exist(&filepath), false); 
    }
}
