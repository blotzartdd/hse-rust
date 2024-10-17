#![forbid(unsafe_code)]

// use itertools::Itertools;

pub fn count_lines_with_word(lines: impl Iterator<Item = String>, word: &str) -> usize {
    let mut count: usize = 0;
    for line in lines {
        if line.to_lowercase().contains(&word.to_lowercase()) {
            count += 1;
        }
    }

    count
}

pub fn top_k_longest(
    lines: impl Iterator<Item = String>,
    k: usize,
) -> impl Iterator<Item = (usize, String)> {
    let mut sorted_lines: Vec<(usize, String)> = Vec::new();

    for (pos, line) in lines.enumerate() {
        sorted_lines.push((pos, line));
    }

    sorted_lines.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    sorted_lines = sorted_lines[0..k].to_vec();
    sorted_lines.into_iter()
}

pub fn words_counter_iter<'a>(
    lines: impl Iterator<Item = String> + 'a,
    word: &'a str,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    let mut res: Vec<(usize, usize)> = Vec::new();
    for (pos, line) in lines.enumerate() {
        let count = line
            .to_lowercase()
            .as_bytes()
            .windows(word.len())
            .filter(|&s| s == word.to_lowercase().as_bytes())
            .count();

        if count > 0 {
            res.push((count, pos));
        }
    }

    res.into_iter()
}
