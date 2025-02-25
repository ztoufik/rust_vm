 
use super::vobj::Vobj;

pub struct Vm<'a> {
    vars: Vec<Vobj<'a>>,
    accu: Vobj<'a>,
}
