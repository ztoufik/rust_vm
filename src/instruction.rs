use super::err::Source;
use super::memloc::MemLoc;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction<'a> {
    Load {
        operand1: MemLoc,
        operand2: Option<MemLoc>,
        str_repr: String,
        src: Source<'a>,
    },
}

impl<'a> Instruction<'a> {
    pub fn load_instruction(operand1: MemLoc, operand2: Option<MemLoc>, src: Source<'a>) -> Self {
        let var1: &str = operand1.ident();
        let mut str_repr: String = format!("Load {}", var1);
        if let Some(ref memeloc) = operand2 {
            str_repr = format!("{} {}", str_repr, memeloc)
        }
        Self::Load {
            operand1,
            operand2,
            str_repr,
            src,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instruction::*;

    #[test]
    fn instruction_type_test() {
        let operand1 = MemLoc::memloc_var("var1".to_owned());
        let operand2 = Option::None;
        let src = Source::new("", 0);
        let variable = Instruction::load_instruction(operand1, operand2, src);
        assert!(matches!(variable,Instruction::Load {..}));
    }

    #[test]
    fn instruction_str_repr_test1() {
        let operand1 = MemLoc::memloc_var("var1".to_owned());
        let operand2 = Option::None;
        let src = Source::new("", 0);
        let variable = Instruction::load_instruction(operand1, operand2, src);
        if let Instruction::Load { str_repr, .. } = variable {
        assert_eq!(str_repr, "Load var1"); // Compare `str_repr` with the expected value
    }
    }

    #[test]
    fn instruction_str_repr_test2() {
        let operand1 = MemLoc::memloc_var("var1".to_owned());
        let operand2 = Option::Some(MemLoc::memloc_var("var2".to_owned()));
        let src = Source::new("", 0);
        let variable = Instruction::load_instruction(operand1, operand2, src);
        if let Instruction::Load { str_repr, .. } = variable {
        assert_eq!(str_repr, "Load var1 var2"); // Compare `str_repr` with the expected value
    }
    }
}
