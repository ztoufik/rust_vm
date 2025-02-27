#[derive(Debug, Clone, PartialEq)]
pub enum Vobj {
    Str(String),
    Double(f64),
    Int(i64),
    Null,
}

impl Vobj {
    pub fn new_str(value: &str) -> Self {
        Vobj::Str(String::from(value))
    }

    pub fn new_double(value: f64) -> Self {
        Vobj::Double(value)
    }

    pub fn new_int(value: i64) -> Self {
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
    use crate::vobj::*;

    #[test]
    fn vobj_string() {
        let variable=Vobj::new_str("test_string");
        assert_eq!(variable, Vobj::Str(String::from("test_string")));
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
