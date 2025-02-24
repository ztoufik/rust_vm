#[derive(Debug, Clone, PartialEq)]
pub enum Vobj {
    Str(String),
    Double(f64),
    Int(i64),
    Null,
}

impl Vobj {
    pub fn new_str(value: &str) -> Vobj {
        Vobj::Str(value.to_owned())
    }

    pub fn new_double(value: f64) -> Vobj {
        Vobj::Double(value)
    }

    pub fn new_int(value: i64) -> Vobj {
        Vobj::Int(value)
    }
}

impl Default for Vobj {
    fn default() -> Self {
        Vobj::Null
    }
}

#[cfg(test)]
mod test{
    use crate::vm::Vobj;

    #[test]
    fn vobj_string() {
        let variable=Vobj::new_str("test_string");
        assert_eq!(variable, Vobj::Str("test_string".to_string()));
    }

    #[test]
    fn vobj_double() {
        let variable=Vobj::new_double(4f64);
        assert_eq!(variable, Vobj::Double(4f64));
    }
    
    #[test]
    fn vobj_int() {
        let variable=Vobj::new_int(4i64);
        assert_eq!(variable, Vobj::Int(4i64));
    }

    #[test]
    fn vobj_default() {
        assert_eq!(Vobj::Null, Vobj::default());
    }
}
