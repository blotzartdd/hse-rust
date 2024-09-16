#![forbid(unsafe_code)]

pub fn is_correct_bracket_sequence(s: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    let check = std::collections::HashMap::from([(')', '('), ('}', '{'), (']', '[')]);

    for c in s.chars() {
        if check.contains_key(&c) {
            if !stack.is_empty() && stack[stack.len() - 1] == check[&c] {
                stack.pop();
            } else {
                return false;
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}
