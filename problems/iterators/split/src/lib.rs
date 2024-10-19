#![forbid(unsafe_code)]

#[derive(Debug)]
pub struct SplitString<'input, 'delimeter: 'input> {
    remainder: Option<&'input str>,
    delimiter: &'delimeter str,
    last_index: usize,
    is_end: bool,
}

impl<'input, 'delimiter> SplitString<'input, 'delimiter> {
    pub fn new(input: &'input str, delimiter: &'delimiter str) -> Self {
        SplitString {
            remainder: Some(input),
            delimiter,
            last_index: 0,
            is_end: false,
        }
    }
}

impl<'input, 'delimiter> Iterator for SplitString<'input, 'delimiter> {
    type Item = &'input str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.delimiter.is_empty() && self.remainder.is_some() {
            if self.last_index >= self.remainder.unwrap().len() {
                return None;
            }

            let c = self
                .remainder
                .as_mut()
                .unwrap()
                .char_indices()
                .nth(0)
                .unwrap()
                .1;

            self.last_index += c.len_utf8();
            return Some(
                &self.remainder.as_mut().unwrap()[self.last_index - c.len_utf8()..self.last_index],
            );
        }

        if self.is_end || self.remainder.is_none() {
            return None;
        }

        let index = self.remainder.as_mut().unwrap()[self.last_index..].find(self.delimiter);
        if index.is_none() {
            self.is_end = true;
            return Some(&self.remainder.as_mut().unwrap()[self.last_index..]);
        }

        let tmp = self.last_index;
        self.last_index = index.unwrap() + self.last_index + self.delimiter.len();

        return Some(
            &self.remainder.as_mut().unwrap()[tmp..self.last_index - self.delimiter.len()],
        );
    }
}

pub fn split<'input, 'delimiter>(
    input: &'input str,
    delimiter: &'delimiter str,
) -> SplitString<'input, 'delimiter> {
    SplitString::new(input, delimiter)
}
