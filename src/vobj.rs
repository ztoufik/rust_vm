use std::mem::discriminant;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Vobj {
    Str(String),
    Double(f64),
    Int(i64),
    Null,
}

impl Vobj {
    pub fn new_str(value: String) -> Self {
        Vobj::Str(value)
    }

    pub fn new_double(value: f64) -> Self {
        Vobj::Double(value)
    }

    pub fn new_int(value: i64) -> Self {
        Vobj::Int(value)
    }

    pub fn same_type(obj1: &Vobj, obj2: &Vobj) -> bool {
        discriminant(obj1) == discriminant(obj2)
    }
}

impl Default for Vobj {
    fn default() -> Self {
        Vobj::Null
    }
}

impl fmt::Display for Vobj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(value) => write!(f, "{}", value),
            Self::Double(value) => write!(f, "{}", value),
            Self::Int(value) => write!(f, "{}", value),
            Self::Null => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vobj::*;

    #[test]
    fn vobj_eq() {
        assert!(Vobj::same_type(
            &Vobj::new_str("4.0".to_owned()),
            &Vobj::new_str("4.0".to_owned())
        ));
    }
}
