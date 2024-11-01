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
