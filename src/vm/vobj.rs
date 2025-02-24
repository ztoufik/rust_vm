#[derive(Debug, Clone, PartialEq)]
pub enum Vobj<'a> {
    Str(&'a str),
    Double(f64),
    Int(i64),
    Null,
}

impl<'a> Vobj<'a> {
    pub fn new_str(value: &'a str) -> Vobj {
        Vobj::Str(value)
    }

    pub fn new_double(value: f64) -> Vobj<'a> {
        Vobj::Double(value)
    }

    pub fn new_int(value: i64) -> Vobj<'a> {
        Vobj::Int(value)
    }
}

impl<'a> Default for Vobj<'a> {
    fn default() -> Self {
        Vobj::Null
    }
}
