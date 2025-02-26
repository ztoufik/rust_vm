 
use super::vobj::Vobj;
use super::instruction::Instruction;

pub struct Vm<'a> {
    vars: Vec<Vobj<'a>>,
    consts: Vec<Vobj<'a>>,
    accu: Vobj<'a>,
    code: Vec<Instruction<'a>>,
}
