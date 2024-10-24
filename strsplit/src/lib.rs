// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a, 'b> {
    remainder: Option<&'a str>,
    delimiter: &'b str
}

impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str)-> Self {
        Self {
            remainder: Some(haystack),
            delimiter
        }
    }
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                return Some(until_delimiter);
            } else {
                return self.remainder.take();
            }
        }
        None
    }
}


fn until_char<'a>(s: &'a str, c: char) -> &'a str {
    StrSplit::new(s, &format!("{}",c)).next().expect("StrSplit always gives at least one result")
}