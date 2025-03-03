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
    pc: Cell<usize>,
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
            pc: Cell::new(0),
        }
    }

    pub fn run(&'a self) -> Result<(), String> {
        let len = self.code.len();
        while self.pc.get() < len {
            let result = match &self.code[self.pc.get()] {
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
                Instruction::AddD {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_addd(operand1), str_repr, src),
                Instruction::SubD {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_subd(operand1), str_repr, src),
                Instruction::MultD {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_multd(operand1), str_repr, src),
                Instruction::DivD {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_divd(operand1), str_repr, src),
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
                Instruction::MultI {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_multi(operand1), str_repr, src),
                Instruction::DivI {
                    operand1,
                    str_repr,
                    src,
                } => (self.execute_divi(operand1), str_repr, src),
                Instruction::Br {
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_br(*br_index), str_repr, src),
                Instruction::Beq {
                    operand,
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_beq(operand, *br_index), str_repr, src),
                Instruction::Bnq {
                    operand,
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_bnq(operand, *br_index), str_repr, src),
                Instruction::Bg {
                    operand,
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_bgt(operand, *br_index), str_repr, src),
                Instruction::Bge {
                    operand,
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_bge(operand, *br_index), str_repr, src),
                Instruction::Blt {
                    operand,
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_blt(operand, *br_index), str_repr, src),
                Instruction::Ble {
                    operand,
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_ble(operand, *br_index), str_repr, src),
                _ => todo!("implement other instructions executions"),
            };
            if let (Err(error), str_repr, src) = result {
                return Err(format!(
                    "{} -> {} {} ({})",
                    error,
                    self.pc.get(),
                    str_repr,
                    src
                ));
            }
            self.pc.set(self.pc.get() + 1);
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

    fn execute_addd(&self, operand1: &MemLoc) -> Result<(), String> {
        let value = self.search_ident(&operand1)?;
        if let Vobj::Double(value1) = *value {
            let vobj_value = if let Vobj::Double(value2) = *self.accu.borrow().clone() {
                Ok(value2)
            } else {
                Err("the accumlator valuemust be of type Double".to_owned())
            };
            if let Ok(value2) = vobj_value {
                *self.accu.borrow_mut() = Rc::new(Vobj::Double(value1 + value2));
                Ok(())
            } else {
                Err("the accumlator valuemust be of type Double".to_owned())
            }
        } else {
            Err(format!("the operand {} must be of type Double", operand1))
        }
    }

    fn execute_subd(&self, operand1: &MemLoc) -> Result<(), String> {
        let value = self.search_ident(&operand1)?;
        if let Vobj::Double(value1) = *value {
            let vobj_value = if let Vobj::Double(value2) = *self.accu.borrow().clone() {
                Ok(value2)
            } else {
                Err("the accumlator valuemust be of type Double".to_owned())
            };
            if let Ok(value2) = vobj_value {
                *self.accu.borrow_mut() = Rc::new(Vobj::Double(value1 - value2));
                Ok(())
            } else {
                Err("the accumlator valuemust be of type Double".to_owned())
            }
        } else {
            Err(format!("the operand {} must be of type Double", operand1))
        }
    }

    fn execute_multd(&self, operand1: &MemLoc) -> Result<(), String> {
        let value = self.search_ident(&operand1)?;
        if let Vobj::Double(value1) = *value {
            let vobj_value = if let Vobj::Double(value2) = *self.accu.borrow().clone() {
                Ok(value2)
            } else {
                Err("the accumlator valuemust be of type Double".to_owned())
            };
            if let Ok(value2) = vobj_value {
                *self.accu.borrow_mut() = Rc::new(Vobj::Double(value1 * value2));
                Ok(())
            } else {
                Err("the accumlator valuemust be of type Double".to_owned())
            }
        } else {
            Err(format!("the operand {} must be of type Double", operand1))
        }
    }

    fn execute_divd(&self, operand1: &MemLoc) -> Result<(), String> {
        let value = self.search_ident(&operand1)?;
        if let Vobj::Double(value1) = *value {
            let vobj_value = if let Vobj::Double(value2) = *self.accu.borrow().clone() {
                Ok(value2)
            } else {
                Err("the accumlator valuemust be of type Double".to_owned())
            };
            if let Ok(value2) = vobj_value {
                if value2 == 0.0 {
                    return Err("Division by zero".to_owned());
                }
                *self.accu.borrow_mut() = Rc::new(Vobj::Double(value1 / value2));
                Ok(())
            } else {
                Err("the accumlator valuemust be of type Double".to_owned())
            }
        } else {
            Err(format!("the operand {} must be of type Double", operand1))
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

    fn execute_multi(&self, operand1: &MemLoc) -> Result<(), String> {
        let value = self.search_ident(&operand1)?;
        if let Vobj::Int(value1) = *value {
            let vobj_value = if let Vobj::Int(value2) = *self.accu.borrow().clone() {
                Ok(value2)
            } else {
                Err("the accumlator valuemust be of type Int".to_owned())
            };
            if let Ok(value2) = vobj_value {
                *self.accu.borrow_mut() = Rc::new(Vobj::Int(value1 * value2));
                Ok(())
            } else {
                Err("the accumlator valuemust be of type Int".to_owned())
            }
        } else {
            Err(format!("the operand {} must be of type Int", operand1))
        }
    }

    fn execute_divi(&self, operand1: &MemLoc) -> Result<(), String> {
        let value = self.search_ident(&operand1)?;
        if let Vobj::Int(value1) = *value {
            let vobj_value = if let Vobj::Int(value2) = *self.accu.borrow().clone() {
                Ok(value2)
            } else {
                Err("the accumlator valuemust be of type Int".to_owned())
            };
            if let Ok(value2) = vobj_value {
                if value2 == 0 {
                    return Err("Division by zero".to_owned());
                }
                *self.accu.borrow_mut() = Rc::new(Vobj::Int(value1 / value2));
                Ok(())
            } else {
                Err("the accumlator valuemust be of type Int".to_owned())
            }
        } else {
            Err(format!("the operand {} must be of type Int", operand1))
        }
    }

    fn execute_br(&self, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        self.pc.set(br_index - 1);
        Ok(())
    }

    fn execute_beq(&self, operand: &MemLoc, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        let value1 = self.search_ident(&operand)?;
        let value2 = self.accu.borrow();
        if *value1 == **value2 {
            self.pc.set(br_index - 1);
        }
        Ok(())
    }

    fn execute_bnq(&self, operand: &MemLoc, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        let value1 = self.search_ident(&operand)?;
        let value2 = self.accu.borrow();
        if *value1 != **value2 {
            self.pc.set(br_index - 1);
        }
        Ok(())
    }

    fn execute_bgt(&self, operand: &MemLoc, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        let value1 = self.search_ident(&operand)?;
        let value2 = self.accu.borrow();
        let result = match &*value1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = **value2 {
                    Ok(*value1 > value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }
            Vobj::Int(value1) => {
                if let Vobj::Int(value2) = **value2 {
                    Ok(*value1 > value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }

            Vobj::Str(value1) => {
                if let Vobj::Str(value2) = &**value2 {
                    Ok(*value1 > *value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }
            Vobj::Null => Err("operand has value Null".to_owned()),
        };
        let result = result?;
        if result {
            self.pc.set(br_index - 1);
        }
        Ok(())
    }

    fn execute_bge(&self, operand: &MemLoc, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        let value1 = self.search_ident(&operand)?;
        let value2 = self.accu.borrow();
        let result = match &*value1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = **value2 {
                    Ok(*value1 >= value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }
            Vobj::Int(value1) => {
                if let Vobj::Int(value2) = **value2 {
                    Ok(*value1 >= value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }

            Vobj::Str(value1) => {
                if let Vobj::Str(value2) = &**value2 {
                    Ok(*value1 >= *value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }
            Vobj::Null => Err("operand has value Null".to_owned()),
        };
        let result = result?;
        if result {
            self.pc.set(br_index - 1);
        }
        Ok(())
    }

    fn execute_blt(&self, operand: &MemLoc, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        let value1 = self.search_ident(&operand)?;
        let value2 = self.accu.borrow();
        let result = match &*value1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = **value2 {
                    Ok(*value1 < value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }
            Vobj::Int(value1) => {
                if let Vobj::Int(value2) = **value2 {
                    Ok(*value1 < value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }

            Vobj::Str(value1) => {
                if let Vobj::Str(value2) = &**value2 {
                    Ok(*value1 < *value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }
            Vobj::Null => Err("operand has value Null".to_owned()),
        };
        let result = result?;
        if result {
            self.pc.set(br_index - 1);
        }
        Ok(())
    }

    fn execute_ble(&self, operand: &MemLoc, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        let value1 = self.search_ident(&operand)?;
        let value2 = self.accu.borrow();
        let result = match &*value1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = **value2 {
                    Ok(*value1 <= value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }
            Vobj::Int(value1) => {
                if let Vobj::Int(value2) = **value2 {
                    Ok(*value1 <= value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }

            Vobj::Str(value1) => {
                if let Vobj::Str(value2) = &**value2 {
                    Ok(*value1 <= *value2)
                } else {
                    Err("both accumlator & operand must have the same type".to_owned())
                }
            }
            Vobj::Null => Err("operand has value Null".to_owned()),
        };
        let result = result?;
        if result {
            self.pc.set(br_index - 1);
        }
        Ok(())
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
        let operand1 = MemLoc::reserve_const("const1".to_owned());
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
        let operand1 = MemLoc::reserve_const("const1".to_owned());
        let var1 = MemLoc::reserve_var("var1".to_owned());
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
    fn vm_addd_test() {
        let operand1 = MemLoc::reserve_const("const1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(operand1.clone(), src.clone());
        let inst2 = Instruction::addd(operand1.clone(), src);
        let code = vec![inst1, inst2];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(0.13)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Double(0.13 * 2.0));
    }

    #[test]
    fn vm_subd_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let operand1 = MemLoc::reserve_var("var1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const2, src.clone());
        let inst2 = Instruction::loadwv_instruction(operand1.clone(), src.clone());
        let inst3 = Instruction::loadw_instruction(const1, src.clone());
        let inst4 = Instruction::subd(operand1.clone(), src);
        let code = vec![inst1, inst2, inst3, inst4];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(0.3)));
        consts.insert("const2", Rc::new(Vobj::Double(0.3)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Double(0.0));
    }

    #[test]
    fn vm_multd_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let operand1 = MemLoc::reserve_var("var1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const2, src.clone());
        let inst2 = Instruction::loadwv_instruction(operand1.clone(), src.clone());
        let inst3 = Instruction::loadw_instruction(const1, src.clone());
        let inst4 = Instruction::multd(operand1.clone(), src);
        let code = vec![inst1, inst2, inst3, inst4];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(0.1)));
        consts.insert("const2", Rc::new(Vobj::Double(15.0)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Double(1.5));
    }

    #[test]
    fn vm_divd_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let operand1 = MemLoc::reserve_var("var1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const2, src.clone());
        let inst2 = Instruction::loadwv_instruction(operand1.clone(), src.clone());
        let inst3 = Instruction::loadw_instruction(const1, src.clone());
        let inst4 = Instruction::divd(operand1.clone(), src);
        let code = vec![inst1, inst2, inst3, inst4];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(10.0)));
        consts.insert("const2", Rc::new(Vobj::Double(1.0)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Double(0.1));
    }

    #[test]
    fn vm_divibyzerod_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let operand1 = MemLoc::reserve_var("var1".to_owned());
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::loadw_instruction(const2, src.clone());
        let inst2 = Instruction::loadwv_instruction(operand1.clone(), src.clone());
        let inst3 = Instruction::loadw_instruction(const1, src.clone());
        let inst4 = Instruction::divd(operand1.clone(), src.clone());
        let code = vec![inst1, inst2, inst3, inst4];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(0.0)));
        consts.insert("const2", Rc::new(Vobj::Double(50.0)));
        let vm = Vm::load(consts, code);
        let result = vm.run();
        if let Err(err_mesg) = result {
            assert_eq!(
                err_mesg,
                format!(
                    "{} -> {} {} ({})",
                    "Division by zero".to_owned(),
                    3,
                    "DivD var1".to_owned(),
                    src
                )
            );
        } else {
            assert!(false);
        };
    }

    #[test]
    fn vm_addi_test() {
        let operand1 = MemLoc::reserve_const("const1".to_owned());
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
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let operand1 = MemLoc::reserve_var("var1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const2, src.clone());
        let inst2 = Instruction::loadwv_instruction(operand1.clone(), src.clone());
        let inst3 = Instruction::loadw_instruction(const1, src.clone());
        let inst4 = Instruction::subi(operand1.clone(), src);
        let code = vec![inst1, inst2, inst3, inst4];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(3)));
        consts.insert("const2", Rc::new(Vobj::Int(5)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Int(2));
    }

    #[test]
    fn vm_multi_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let operand1 = MemLoc::reserve_var("var1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const2, src.clone());
        let inst2 = Instruction::loadwv_instruction(operand1.clone(), src.clone());
        let inst3 = Instruction::loadw_instruction(const1, src.clone());
        let inst4 = Instruction::multi(operand1.clone(), src);
        let code = vec![inst1, inst2, inst3, inst4];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(3)));
        consts.insert("const2", Rc::new(Vobj::Int(5)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Int(15));
    }

    #[test]
    fn vm_divi_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let operand1 = MemLoc::reserve_var("var1".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const2, src.clone());
        let inst2 = Instruction::loadwv_instruction(operand1.clone(), src.clone());
        let inst3 = Instruction::loadw_instruction(const1, src.clone());
        let inst4 = Instruction::divi(operand1.clone(), src);
        let code = vec![inst1, inst2, inst3, inst4];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(10)));
        consts.insert("const2", Rc::new(Vobj::Int(50)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Int(5));
    }

    #[test]
    fn vm_divibyzeroi_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let operand1 = MemLoc::reserve_var("var1".to_owned());
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::loadw_instruction(const2, src.clone());
        let inst2 = Instruction::loadwv_instruction(operand1.clone(), src.clone());
        let inst3 = Instruction::loadw_instruction(const1, src.clone());
        let inst4 = Instruction::divi(operand1.clone(), src.clone());
        let code = vec![inst1, inst2, inst3, inst4];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(0)));
        consts.insert("const2", Rc::new(Vobj::Int(50)));
        let vm = Vm::load(consts, code);
        let result = vm.run();
        if let Err(err_mesg) = result {
            assert_eq!(
                err_mesg,
                format!(
                    "{} -> {} {} ({})",
                    "Division by zero".to_owned(),
                    3,
                    "DivI var1".to_owned(),
                    src
                )
            );
        } else {
            assert!(false);
        };
    }

    #[test]
    fn vm_br_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let const3 = MemLoc::reserve_const("const3".to_owned());
        let const4 = MemLoc::reserve_const("const4".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const1, src.clone());
        let inst2 = Instruction::loadw_instruction(const2, src.clone());
        let inst3 = Instruction::br(3, src.clone());
        let inst4 = Instruction::loadw_instruction(const3, src.clone());
        let inst5 = Instruction::loadw_instruction(const4, src.clone());
        let code = vec![inst1, inst2, inst3, inst4, inst5];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(10)));
        consts.insert("const2", Rc::new(Vobj::Int(20)));
        consts.insert("const3", Rc::new(Vobj::Int(30)));
        consts.insert("const4", Rc::new(Vobj::Int(40)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Int(40));
    }

    #[test]
    fn vm_beq_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let cond = MemLoc::reserve_const("cond".to_owned());
        let const4 = MemLoc::reserve_const("const4".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const1, src.clone());
        let inst2 = Instruction::addi(const2.clone(), src.clone());
        let inst3 = Instruction::beq(cond, 4, src.clone());
        let inst4 = Instruction::addi(const2, src.clone());
        let inst5 = Instruction::addi(const4, src.clone());
        let code = vec![inst1, inst2, inst3, inst4, inst5];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Int(10)));
        consts.insert("const2", Rc::new(Vobj::Int(20)));
        consts.insert("cond", Rc::new(Vobj::Int(30)));
        consts.insert("const4", Rc::new(Vobj::Int(40)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Int(70));
    }

    #[test]
    fn vm_bnq_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let cond = MemLoc::reserve_const("cond".to_owned());
        let const4 = MemLoc::reserve_const("const4".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const1, src.clone());
        let inst2 = Instruction::addd(const2.clone(), src.clone());
        let inst3 = Instruction::bnq(cond, 4, src.clone());
        let inst4 = Instruction::addd(const2, src.clone());
        let inst5 = Instruction::addd(const4, src.clone());
        let code = vec![inst1, inst2, inst3, inst4, inst5];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(10.0)));
        consts.insert("const2", Rc::new(Vobj::Double(20.0)));
        consts.insert("cond", Rc::new(Vobj::Double(0.0)));
        consts.insert("const4", Rc::new(Vobj::Double(40.0)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Double(70.0));
    }

    #[test]
    fn vm_bgt_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let cond = MemLoc::reserve_const("cond".to_owned());
        let const4 = MemLoc::reserve_const("const4".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const1, src.clone());
        let inst2 = Instruction::addd(const2.clone(), src.clone());
        let inst3 = Instruction::bgt(cond, 4, src.clone());
        let inst4 = Instruction::addd(const2, src.clone());
        let inst5 = Instruction::addd(const4, src.clone());
        let code = vec![inst1, inst2, inst3, inst4, inst5];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(10.0)));
        consts.insert("const2", Rc::new(Vobj::Double(20.0)));
        consts.insert("cond", Rc::new(Vobj::Double(30.1)));
        consts.insert("const4", Rc::new(Vobj::Double(40.0)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Double(70.0));
    }

    #[test]
    fn vm_bge_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let cond = MemLoc::reserve_const("cond".to_owned());
        let const4 = MemLoc::reserve_const("const4".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const1, src.clone());
        let inst2 = Instruction::addd(const2.clone(), src.clone());
        let inst3 = Instruction::bge(cond, 4, src.clone());
        let inst4 = Instruction::addd(const2, src.clone());
        let inst5 = Instruction::addd(const4, src.clone());
        let code = vec![inst1, inst2, inst3, inst4, inst5];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(10.0)));
        consts.insert("const2", Rc::new(Vobj::Double(20.0)));
        consts.insert("cond", Rc::new(Vobj::Double(30.1)));
        consts.insert("const4", Rc::new(Vobj::Double(40.0)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Double(70.0));
    }

    #[test]
    fn vm_blt_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let cond = MemLoc::reserve_const("cond".to_owned());
        let const4 = MemLoc::reserve_const("const4".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const1, src.clone());
        let inst2 = Instruction::addd(const2.clone(), src.clone());
        let inst3 = Instruction::blt(cond, 4, src.clone());
        let inst4 = Instruction::addd(const2, src.clone());
        let inst5 = Instruction::addd(const4, src.clone());
        let code = vec![inst1, inst2, inst3, inst4, inst5];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(10.0)));
        consts.insert("const2", Rc::new(Vobj::Double(20.0)));
        consts.insert("cond", Rc::new(Vobj::Double(10.0)));
        consts.insert("const4", Rc::new(Vobj::Double(40.0)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Double(70.0));
    }

    #[test]
    fn vm_ble_test() {
        let const1 = MemLoc::reserve_const("const1".to_owned());
        let const2 = MemLoc::reserve_const("const2".to_owned());
        let cond = MemLoc::reserve_const("cond".to_owned());
        let const4 = MemLoc::reserve_const("const4".to_owned());
        let src = Source::new("", 0);
        let inst1 = Instruction::loadw_instruction(const1, src.clone());
        let inst2 = Instruction::addd(const2.clone(), src.clone());
        let inst3 = Instruction::ble(cond, 4, src.clone());
        let inst4 = Instruction::addd(const2, src.clone());
        let inst5 = Instruction::addd(const4, src.clone());
        let code = vec![inst1, inst2, inst3, inst4, inst5];
        let mut consts = HashMap::new();
        consts.insert("const1", Rc::new(Vobj::Double(10.0)));
        consts.insert("const2", Rc::new(Vobj::Double(20.0)));
        consts.insert("cond", Rc::new(Vobj::Double(30.0)));
        consts.insert("const4", Rc::new(Vobj::Double(40.0)));
        let mut vm = Vm::load(consts, code);
        vm.run();
        assert_eq!(**(vm.accu.get_mut()), Vobj::Double(70.0));
    }
}
