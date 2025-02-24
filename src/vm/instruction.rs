use self::err::source;
pub enum Instruction {
    Load(vec<MemLoc>, String, source),
}

impl Instruction {
    pub fn load_instruction(operands: vec<MemLoc>, str_repr: &str, src: source) -> Instruction {
        Self::Load(operands, str_repr.to_owned(), src)
    }
}
