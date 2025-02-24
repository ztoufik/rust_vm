pub mod vobj;
pub use vobj::Vobj;

pub struct Vm {
    vars: Vec<Vobj>,
    accu: Vobj,
}
