use std::cell::{Cell, Ref, RefCell};
use std::vec::Vec;

use super::instruction::Instruction;
use super::vobj::Vobj;

pub struct Vm<'a> {
    stack: RefCell<Vec<Vobj>>,
    code: Vec<Instruction<'a>>,
    pc: Cell<usize>,
}

impl<'a> Vm<'a> {
    pub fn dump_mem(&self) -> Ref<'_, Vec<Vobj>> {
        self.stack.borrow()
    }

    pub fn load(code: Vec<Instruction<'a>>) -> Self {
        let stack = RefCell::new(Vec::new());
        Self {
            stack,
            code,
            pc: Cell::new(0),
        }
    }

    pub fn run(&'a self) -> Result<(), String> {
        let len = self.code.len();
        while self.pc.get() < len {
            let result = match &self.code[self.pc.get()] {
                Instruction::Nop { str_repr, src } => (Ok(()),str_repr,src),
                Instruction::Load {
                    value,
                    str_repr,
                    src,
                } => (self.execute_load((*value).clone()), str_repr, src),
                Instruction::Add { str_repr, src } => (self.execute_add(), str_repr, src),
                Instruction::Sub { str_repr, src } => (self.execute_sub(), str_repr, src),
                Instruction::Mul { str_repr, src } => (self.execute_mul(), str_repr, src),
                Instruction::Div { str_repr, src } => (self.execute_div(), str_repr, src),
                Instruction::Br {
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_br(*br_index), str_repr, src),
                Instruction::Beq {
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_beq(*br_index), str_repr, src),
                Instruction::Bnq {
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_bnq(*br_index), str_repr, src),
                Instruction::Bg {
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_bgt(*br_index), str_repr, src),
                Instruction::Bge {
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_bge(*br_index), str_repr, src),
                Instruction::Blt {
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_blt(*br_index), str_repr, src),
                Instruction::Ble {
                    br_index,
                    str_repr,
                    src,
                } => (self.execute_ble(*br_index), str_repr, src),
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

    fn execute_load(&self, value: Vobj) -> Result<(), String> {
        self.stack.borrow_mut().push(value);
        Ok(())
    }

    fn execute_add(&self) -> Result<(), String> {
        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let sum = Vobj::add(&value1, &value2);
        match sum {
            Ok(result) => {
                self.stack.borrow_mut().push(result);
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    }

    fn execute_sub(&self) -> Result<(), String> {
        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let sum = Vobj::sub(&value1, &value2);
        match sum {
            Ok(result) => {
                self.stack.borrow_mut().push(result);
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    }

    fn execute_mul(&self) -> Result<(), String> {
        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let sum = Vobj::mul(&value1, &value2);
        match sum {
            Ok(result) => {
                self.stack.borrow_mut().push(result);
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    }

    fn execute_div(&self) -> Result<(), String> {
        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let sum = Vobj::div(&value1, &value2);
        match sum {
            Ok(result) => {
                self.stack.borrow_mut().push(result);
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    }

    fn execute_br(&self, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        self.pc.set(br_index - 1);
        Ok(())
    }

    fn execute_beq(&self, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        if value1 == value2 {
            self.pc.set(br_index - 1);
        }
        Ok(())
    }

    fn execute_bnq(&self, br_index: usize) -> Result<(), String> {
        if br_index >= self.code.len() {
            return Err("invalid instruction index: out of bound".to_owned());
        }

        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        if value1 != value2 {
            self.pc.set(br_index - 1);
        }
        Ok(())
    }

    fn execute_bgt(&self, br_index: usize) -> Result<(), String> {
        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let sum = Vobj::greater_than(&value1, &value2);
        match sum {
            Ok(result) => {
                if result {
                    self.pc.set(br_index - 1);
                }
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    }

    fn execute_bge(&self, br_index: usize) -> Result<(), String> {
        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let sum = Vobj::greater_eq(&value1, &value2);
        match sum {
            Ok(result) => {
                if result {
                    self.pc.set(br_index - 1);
                }
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    }

    fn execute_blt(&self, br_index: usize) -> Result<(), String> {
        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let sum = Vobj::less_than(&value1, &value2);
        match sum {
            Ok(result) => {
                if result {
                    self.pc.set(br_index - 1);
                }
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    }

    fn execute_ble(&self, br_index: usize) -> Result<(), String> {
        if self.stack.borrow().len() < 2 {
            return Err("at least 2 argument are required".to_string());
        }
        let value2 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let value1 = self.stack.borrow_mut().pop().expect("empty runtime stack");
        let sum = Vobj::less_eq(&value1, &value2);
        match sum {
            Ok(result) => {
                if result {
                    self.pc.set(br_index - 1);
                }
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    }
}
