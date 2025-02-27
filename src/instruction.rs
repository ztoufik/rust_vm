use super::err::Source;
use super::memloc::MemLoc;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction<'a> {
    LoadVV {
        operand1: MemLoc,
        operand2: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

    LoadCV {
        operand1: MemLoc,
        operand2: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

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

    pub fn loadVV_instruction(operand1: MemLoc, operand2: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let var2: &str = operand2.ident();
        let str_repr: String = format!("LoadVV {} {}", var1,var2);
        Self::LoadVV {
            operand1,
            operand2,
            str_repr,
            src,
        }
    }

    pub fn loadCV_instruction(operand1: MemLoc, operand2: MemLoc, src: Source<'a>) -> Self {
        let const1: &str = operand1.ident();
        let var2: &str = operand2.ident();
        let str_repr: String = format!("LoadCV {} {}", const1,var2);
        Self::LoadCV {
            operand1,
            operand2,
            str_repr,
            src,
        }
    }

    pub fn loadWV_instruction(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("LoadWV {}", var1);
        Self::LoadWV {
            operand1,
            str_repr,
            src,
        }
    }

    pub fn loadW_instruction(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("LoadW {}", var1);
        Self::LoadW {
            operand1,
            str_repr,
            src,
        }
    }

}
