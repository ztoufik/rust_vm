#[derive(Debug, Clone, PartialEq)]
pub enum MemLoc<'a> {
    Const(&'a str),
    Var(&'a str),
}

impl<'a> MemLoc<'a> {
    pub fn memloc_const(value: &'a str) -> Self {
        MemLoc::Const(value)
    }

    pub fn memloc_var(value: &'a str) -> Self {
        MemLoc::Var(value)
    }
}

#[cfg(test)]
mod test {
    use crate::memloc::*;

    #[test]
    fn memloc_const() {
        let variable=MemLoc::Const("const");
        assert_eq!(variable, MemLoc::Const("const"));
    }

    #[test]
    fn memloc_var() {
        let variable=MemLoc::Var("var");
        assert_eq!(variable, MemLoc::Var("var"));
    }

}
