use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum MemLoc {
    Const(String),
    Var(String),
}

impl MemLoc {
    pub fn reserve_const(value:String) -> Self {
        MemLoc::Const(value)
    }

    pub fn reserve_var(value:String) -> Self {
        MemLoc::Var(value)
    }


    pub fn ident(&self)->&str{
        match self {
            MemLoc::Const(value)=> value.as_str(),
            MemLoc::Var(value)=> value.as_str(),
        }
    }
}

impl fmt::Display for MemLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the formatted string to the formatter
        write!(f, "{}", self.ident())
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
