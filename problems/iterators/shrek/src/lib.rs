#![forbid(unsafe_code)]

use itertools::Itertools;

pub fn count_lines_with_word(lines: impl Iterator<Item = String>, word: &str) -> usize {
    // TODO: your code goes here.
    unimplemented!()
}

pub fn top_k_longest(
    lines: impl Iterator<Item = String>,
    k: usize,
) -> impl Iterator<Item = (usize, String)> {
    // TODO: your code goes here.
    unimplemented!()
}

pub fn words_counter_iter<'a>(
    lines: impl Iterator<Item = String> + 'a,
    word: &'a str,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    // TODO: your code goes here.
    unimplemented!()
}
