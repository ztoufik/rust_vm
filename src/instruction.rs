use super::err::Source;
use super::vobj::Vobj;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction<'a> {
    Nop {
        str_repr: String,
        src: Source<'a>,
    },

    Load {
        value:Vobj,
        str_repr: String,
        src: Source<'a>,
    },

    Add {
        str_repr: String,
        src: Source<'a>,
    },

    Sub {
        
        str_repr: String,
        src: Source<'a>,
    },

    Mul {
        
        str_repr: String,
        src: Source<'a>,
    },

    Div {
        
        str_repr: String,
        src: Source<'a>,
    },

    Br {
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Beq {
        
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Bnq {
        
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Bg {
        
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Bge {
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Blt {
        
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },

    Ble {
        
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },
    Bgt {
        
        br_index: usize,
        str_repr: String,
        src: Source<'a>,
    },
}

impl<'a> Instruction<'a> {
    pub fn nop_instruction(src: Source<'a>) -> Self {
        let str_repr: String = "Load".to_string();
        Self::Nop {
            str_repr,
            src,
        }
    }
    pub fn load_instruction(value:Vobj,src: Source<'a>) -> Self {
        let str_repr: String = "Load".to_string();
        Self::Load {
            value,
            str_repr,
            src,
        }
    }

    pub fn add(src: Source<'a>) -> Self {
        let str_repr: String = "Add".to_string();
        Self::Add {
            str_repr,
            src,
        }
    }

    pub fn sub(src: Source<'a>) -> Self {
        let str_repr: String = "Sub".to_string();
        Self::Sub {
            str_repr,
            src,
        }
    }

    pub fn mul(src: Source<'a>) -> Self {
        let str_repr: String = "Mul".to_string();
        Self::Mul {
            str_repr,
            src,
        }
    }

    pub fn div(src: Source<'a>) -> Self {
        let str_repr: String = "Div".to_string();
        Self::Div {
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

    pub fn beq(br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Beq {}", var1);
        Self::Beq {
            br_index,
            str_repr,
            src,
        }
    }

    pub fn bnq(br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Bnq {}", var1);
        Self::Bnq {

            br_index,
            str_repr,
            src,
        }
    }

    pub fn bgt(br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Bgt {}", var1);
        Self::Bg {

            br_index,
            str_repr,
            src,
        }
    }

    pub fn bge(br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Bge {}", var1);
        Self::Bge {
            br_index,
            str_repr,
            src,
        }
    }

    pub fn blt(br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Blt {}", var1);
        Self::Blt {
            br_index,
            str_repr,
            src,
        }
    }

    pub fn ble(br_index: usize, src: Source<'a>) -> Self {
        let var1 = br_index.to_string();
        let str_repr: String = format!("Ble {}", var1);
        Self::Ble {
            br_index,
            str_repr,
            src,
        }
    }
}
