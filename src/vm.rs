use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

use super::instruction::Instruction;
use super::memloc::MemLoc;
use super::vobj::Vobj;

pub struct Vm<'a> {
    vars: RefCell<HashMap<&'a str, Rc<Vobj>>>,
    consts: HashMap<&'a str, Rc<Vobj>>,
    accu: RefCell<Rc<Vobj>>,
    code: Vec<Instruction<'a>>,
}

impl<'a> Vm<'a> {
    pub fn load(consts: HashMap<&'a str, Rc<Vobj>>, code: Vec<Instruction<'a>>) -> Self {
        let vars = RefCell::new(HashMap::new());
        let accu = RefCell::new(Rc::new(Vobj::Null));
        Self {
            vars,
            consts,
            accu,
            code,
        }
    }

    pub fn run(&'a self) -> Result<(), String> {
        let len = self.code.len();
        for index in 0..len {
            //let result = match self.code[index].clone() {
            let result = match &self.code[index] {
                Instruction::LoadW {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_load_w(operand1), str_repr, src),
                Instruction::LoadWV {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_load_wv(operand1), str_repr, src),
                Instruction::AddI {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_addi(operand1), str_repr, src),
                Instruction::SubI {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_subi(operand1), str_repr, src),
                _ => todo!("implement other instructions executions"),
            };
            if let (Err(error), str_repr, src) = result {
                return Err(format!("{} {}: at {} {}", str_repr, error, src, index));
            }
        }
        Ok(())
    }

    fn execute_load_w(&self, operand1: &MemLoc) -> Result<(), String> {
        let value1 = self.search_ident(&operand1)?;
        *self.accu.borrow_mut() = value1.clone();
        Ok(())
    }

    fn execute_load_wv(&self, operand1: &'a MemLoc) -> Result<(), String> {
        match operand1 {
            MemLoc::Var(ident) => {
                self.vars
                    .borrow_mut()
                    .insert(ident, self.accu.borrow().clone());
                Ok(())
            }
            MemLoc::Const(ident) => Err(format!("{} expected variable, found const", ident)),
        }
    }

    fn execute_addi(&self, operand1: &MemLoc) -> Result<(), String> {
        let value = self.search_ident(&operand1)?;
        if let Vobj::Int(value1) = *value {
            let vobj_value = if let Vobj::Int(value2) = *self.accu.borrow().clone() {
                Ok(value2)
            } else {
                Err("the accumlator valuemust be of type Int".to_owned())
            };
            if let Ok(value2) = vobj_value {
                *self.accu.borrow_mut() = Rc::new(Vobj::Int(value1 + value2));
                Ok(())
            } else {
                Err("the accumlator valuemust be of type Int".to_owned())
            }
        } else {
            Err(format!("the operand {} must be of type Int", operand1))
        }
    }

    fn execute_subi(&self, operand1: &MemLoc) -> Result<(), String> {
        let value = self.search_ident(&operand1)?;
        if let Vobj::Int(value1) = *value {
            let vobj_value = if let Vobj::Int(value2) = *self.accu.borrow().clone() {
                Ok(value2)
            } else {
                Err("the accumlator valuemust be of type Int".to_owned())
            };
            if let Ok(value2) = vobj_value {
                *self.accu.borrow_mut() = Rc::new(Vobj::Int(value1 - value2));
                Ok(())
            } else {
                Err("the accumlator valuemust be of type Int".to_owned())
            }
        } else {
            Err(format!("the operand {} must be of type Int", operand1))
        }
    }

    fn search_ident(&self, mem_loc: &MemLoc) -> Result<Rc<Vobj>, String> {
        match mem_loc {
            MemLoc::Const(ident) => {
                let ident: &str = &ident;
                if let Some(value) = self.consts.get(ident) {
                    Ok(value.clone())
                } else {
                    Err(format!("{} not found", mem_loc))
                }
            }
            MemLoc::Var(ident) => {
                let ident: &str = &ident;
                if let Some(value) = self.vars.borrow().get(ident).clone() {
                    Ok(value.clone())
                } else {
                    Err(format!("{} not found", mem_loc))
                }
            }
        }
    }

}

#[cfg(test)]
mod test {
    use crate::err::*;
    use crate::vm::*;

    #[test]
    fn vm_loadw_test() {
        let operand1 = MemLoc::memloc_const("const1".to_owned());
        let src = Source::new("", 0);
        let inst = Instruction::loadw_instruction(operand1.clone(), src);
        let code = vec![inst];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(3)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Int(3));
    }

    #[test]
    fn vm_loadwm_test() {
        let operand1 = MemLoc::memloc_const("const1".to_owned());
        let var1 = MemLoc::memloc_var("var1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(operand1, src.clone());
        let inst2 = Instruction::loadwv_instruction(var1, src);
        let code = vec![inst1, inst2];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(3)));
        let vm = Vm::load(consts, code);
        vm.run();
        if let Some(value) = vm.vars.borrow().get("var1") {
            print!("\n****************{}***********\n", value);
            assert_eq!(**value, Vobj::Int(3));
        } else {
            assert!(false);
        };
    }

    #[test]
    fn vm_addi_test() {
        let operand1 = MemLoc::memloc_const("const1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(operand1.clone(), src.clone());
        let inst2 = Instruction::addi(operand1.clone(), src);
        let code = vec![inst1, inst2];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(3)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Int(6));
    }

    #[test]
    fn vm_subi_test() {
        let const1 = MemLoc::memloc_const("const1".to_owned());
        let const2 = MemLoc::memloc_const("const2".to_owned());
        let operand1 = MemLoc::memloc_var("var1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const2, src.clone());
        let inst2 = Instruction::loadwv_instruction(operand1.clone(), src.clone());
        let inst3 = Instruction::loadw_instruction(const1, src.clone());
        let inst4 = Instruction::subi(operand1.clone(), src);
        let code = vec![inst1, inst2,inst3,inst4];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(3)));
        consts.insert("const2", Rc::new(Vobj::Int(5)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Int(2));
    }
}
