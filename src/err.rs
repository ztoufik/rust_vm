use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub struct Source<'a> {
    source_file: &'a str,
    line_number: u32,
}

impl<'a> Source<'a> {
    pub fn new(source_file: &'a str, line_number: u32) -> Self {
        Self {
            source_file,
            line_number,
        }
    }
}

impl<'a> fmt::Display for Source<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the formatted string to the formatter
        write!(f, "{}:{}", self.source_file, self.line_number)
    }
}

#[derive(Debug, Clone,PartialEq,Eq)]
pub enum VMError {
    IncorrectArgumentErr(String),
    DivisionByZeroErr,
}
