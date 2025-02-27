use std::borrow::Borrow;
use std::collections::HashMap;
use std::vec::Vec;

use super::err::Source;
use super::instruction::Instruction;
use super::memloc::MemLoc;
use super::vobj::Vobj;

pub struct Vm<'a> {
    vars: HashMap<&'a str, Vobj>,
    consts: HashMap<&'a str, Vobj>,
    accu: Vobj,
    code: Vec<Instruction<'a>>,
}

impl<'a> Vm<'a> {
    pub fn load(consts: HashMap<&'a str, Vobj>, code: Vec<Instruction<'a>>) -> Self {
        let vars: HashMap<&'a str, Vobj> = HashMap::new();
        let accu = Vobj::default();
        Self {
            vars,
            consts,
            accu,
            code,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let len=self.code.len();
        for index in 0..len{
            let result= match self.code[len].clone() {
                Instruction::LoadW {
                    operand1,
                    str_repr,
                    src,
                } =>  (self.execute_load_w(&operand1),str_repr,src),
                _=> todo!("implement other instructions executions"),

            };
            if let (Err(error),str_repr,src)=result{
                return Err(format!("{} {}: at {} {}",str_repr,error, src, index))
            }
        }
        Ok(())
    }

    fn execute_load_w(
        &mut self,
        operand1: &MemLoc,
    ) -> Result<(), String> {
        let value1=self.search_ident(&operand1)?;
        self.accu=value1.clone();
        Ok(())
    }

    fn execute_load_wm(
        &mut self,
        operand1: &'a MemLoc,
    ) -> Result<(), String> {
        match operand1{
            MemLoc::Var(ident)=>{
                self.vars.insert(ident, self.accu.clone());
                Ok(())
            },
            MemLoc::Const(ident)=> Err(format!("{} expected variable, found const",ident)),
        }
    }

    fn search_ident(&self, mem_loc: &MemLoc) -> Result<&Vobj, String> {
        let ovalue=match mem_loc{
            MemLoc::Const(ident)=>{
                let ident:&str=&ident;
                self.consts.get(ident)
            },
            MemLoc::Var(ident)=>{
                let ident:&str=&ident;
                self.vars.get(ident)
            },
        };

        if let Some(value) = ovalue {
            Ok(value)
        } else {
            Err(format!("{} not found", mem_loc))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vm::*;

    fn return_instruction<'a>() -> Instruction<'a> {
        let operand1 = MemLoc::memloc_var("var1".to_owned());
        let operand2 = Option::None;
        let src = Source::new("", 0);
        Instruction::load_instruction(operand1, operand2, src)
    }

    fn return_instructions<'a>() -> Vec<Instruction<'a>> {
        let mut insts: Vec<Instruction> = Vec::new();
        for i in 0..5 {
            let operand1 = MemLoc::memloc_var(format!("var1{}", i));
            let operand2 = if i % 2 == 0 {
                Option::Some(MemLoc::memloc_var(format!("var2{}", i)))
            } else {
                Option::None
            };
            let src = Source::new("", 0);
            insts.push(Instruction::load_instruction(operand1, operand2, src));
        }
        insts
    }

    fn return_consts() -> HashMap<&str, Vobj> {
        let mut consts: HashMap<&str, Vobj> = HashMap::new();
        consts.insert("const1", Vobj::new_str("value1"));
        consts.insert("const2", Vobj::new_str("value2"));
        consts.insert("const3", Vobj::new_str("value3"));
        consts
    }

    #[test]
    fn vm_load_test() {}
}
