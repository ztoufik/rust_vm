use super::vobj::Vobj;
use super::instruction::Instruction;

#[derive(Debug,Clone,PartialEq)]
pub struct Program<'a> {
    consts: Vec<Vobj<'a>>,
    code: Vec<Instruction>,
}
