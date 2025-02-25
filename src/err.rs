#[derive(Debug, Clone, PartialEq)]
pub struct Source<'a> {
    source_file: &'a str,
    line_number: u32,
}

impl<'a> Source<'a>{
   pub fn new(source_file:&'a str,line_number:u32)->Self{
    Self { source_file, line_number }
   }
}
