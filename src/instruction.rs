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

    AddD {
        operand1: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

    SubD {
        operand1: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

    MultD {
        operand1: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

    DivD {
        operand1: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

    AddI {
        operand1: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

    SubI {
        operand1: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

    MultI {
        operand1: MemLoc,
        str_repr: String,
        src: Source<'a>,
    },

    DivI {
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
    pub fn addi(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("AddI {}", var1);
        Self::AddI {
            operand1,
            str_repr,
            src,
        }
    }

    pub fn subi(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("SubI {}", var1);
        Self::SubI {
            operand1,
            str_repr,
            src,
        }
    }

    pub fn multi(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("MultI {}", var1);
        Self::MultI {
            operand1,
            str_repr,
            src,
        }
    }

    pub fn divi(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("DivI {}", var1);
        Self::DivI {
            operand1,
            str_repr,
            src,
        }
    }

    pub fn addd(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("AddD {}", var1);
        Self::AddD {
            operand1,
            str_repr,
            src,
        }
    }

    pub fn subd(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("SubD {}", var1);
        Self::SubD {
            operand1,
            str_repr,
            src,
        }
    }

    pub fn multd(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("MultD {}", var1);
        Self::MultD {
            operand1,
            str_repr,
            src,
        }
    }

    pub fn divd(operand1: MemLoc, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let str_repr: String = format!("DivD {}", var1);
        Self::DivD {
            operand1,
            str_repr,
            src,
        }
    }

}
