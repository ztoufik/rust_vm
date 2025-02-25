use super::err::Source;
use super::memloc::MemLoc;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction<'a> {
    Load(Vec<MemLoc<'a>>, String, Source<'a>),
}

impl<'a> Instruction<'a> {
    pub fn load_instruction(operands: Vec<MemLoc<'a>>, str_repr: &'a str, src: Source<'a>) -> Self {
        Self::Load(operands, str_repr.to_owned(), src)
    }
}
