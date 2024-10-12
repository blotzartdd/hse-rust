#![forbid(unsafe_code)]

#[derive(Debug)]
pub struct SplitString {
    remainder: Option<&str>,
    delimiter: &str,
}

impl SplitString {
    pub fn new(input: &str, delimiter: &str) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl Iterator for SplitString {
    type Item = &str;
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: your code goes here.
        unimplemented!()
    }
}

pub fn split<'input, 'delimiter>(
    input: &'input str,
    delimiter: &'delimiter str,
) -> SplitString {
    // TODO: your code goes here.
    unimplemented!()
}
