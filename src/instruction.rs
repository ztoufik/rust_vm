use super::err::Source;
use super::memloc::MemLoc;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Load(Vec<MemLoc>, String, Source),
}

impl Instruction {
    pub fn load_instruction(operands: Vec<MemLoc>, str_repr: &str, src: Source) -> Self {
        Self::Load(operands, str_repr.to_owned(), src)
    }
}
