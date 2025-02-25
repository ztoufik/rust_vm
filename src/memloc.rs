#[derive(Debug, Clone, PartialEq)]
pub enum MemLoc {
    Const(String),
    Var(String),
}

impl MemLoc {
    pub fn memloc_const(value: &str) -> MemLoc {
        MemLoc::Const(value.to_owned())
    }

    pub fn memloc_var(value: &str) -> MemLoc {
        MemLoc::Var(value.to_owned())
    }
}

#[cfg(test)]
mod test {
    use crate::memloc::*;

    #[test]
    fn memloc_const() {
        let variable=MemLoc::Const("const".to_owned());
        assert_eq!(variable, MemLoc::Const("const".to_owned()));
    }

    #[test]
    fn memloc_var() {
        let variable=MemLoc::Var("var".to_owned());
        assert_eq!(variable, MemLoc::Var("var".to_owned()));
    }

}
