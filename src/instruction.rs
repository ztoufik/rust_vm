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

    Br {
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Beq {
        operand: MemLoc,
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Bnq {
        operand: MemLoc,
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Bg {
        operand: MemLoc,
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Bge {
        operand: MemLoc,
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Blt {
        operand: MemLoc,
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Ble {
        operand: MemLoc,
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },
    Bgt {
        operand: MemLoc,
        br_index: usize,
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

    pub fn br(br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Br {}", var1);
        Self::Br {
            br_index,
            str_repr,
            src,
        }
    }

    pub fn beq(operand: MemLoc, br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Beq {}", var1);
        Self::Beq {
            operand,
            br_index,
            str_repr,
            src,
        }
    }

    pub fn bnq(operand: MemLoc, br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Bnq {}", var1);
        Self::Bnq {
            operand,
            br_index,
            str_repr,
            src,
        }
    }

    pub fn bgt(operand: MemLoc, br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Bgt {}", var1);
        Self::Bg {
            operand,
            br_index,
            str_repr,
            src,
        }
    }

    pub fn bge(operand: MemLoc, br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Bge {}", var1);
        Self::Bge {
            operand,
            br_index,
            str_repr,
            src,
        }
    }

    pub fn blt(operand: MemLoc, br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Blt {}", var1);
        Self::Blt {
            operand,
            br_index,
            str_repr,
            src,
        }
    }

    pub fn ble(operand: MemLoc, br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Ble {}", var1);
        Self::Ble {
            operand,
            br_index,
            str_repr,
            src,
        }
    }
}
