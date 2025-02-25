#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    source_file: String,
    line_number: u32,
}

impl Source{
   pub fn new(source_file:&str,line_number:u32)->Self{
       let source_file = source_file.to_owned();
    Self { source_file, line_number: line_number }
   }
}
