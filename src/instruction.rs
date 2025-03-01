use super::err::Source;
use super::memloc::MemLoc;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction<'a> {
    LoadWV {
        operand1: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

    LoadW {
        operand1: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },
}

impl<'a> Instruction<'a> {

    pub fn loadwv_instruction(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("LoadWV {}", var1);
        Self::LoadWV {
            operand1,
            str_repr,
            src,
        }
    }

    pub fn loadw_instruction(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("LoadW {}", var1);
        Self::LoadW {
            operand1,
            str_repr,
            src,
        }
    }

}
