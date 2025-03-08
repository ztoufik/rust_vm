use std::fmt;

use crate::err::VMError;

#[derive(Debug, Clone, PartialEq)]
pub enum Vobj {
    Str(String),
    Double(f64),
    Int(i64),
    Null,
}

impl Vobj {
    pub fn add(oprand1: &Vobj, oprand2: &Vobj) -> Result<Vobj, VMError> {
        match *oprand1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(Vobj::Double(value1 + value2))
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(Vobj::Double(value1 + value2 as f64))
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }

            Vobj::Int(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(Vobj::Double(value1 as f64 + value2))
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(Vobj::Int(value1 + value2))
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }
            _ => Err(VMError::IncorrectArgumentErr(String::from(
                "Expected numerical type",
            ))),
        }
    }

    pub fn sub(oprand1: &Vobj, oprand2: &Vobj) -> Result<Vobj, VMError> {
        match *oprand1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(Vobj::Double(value1 - value2))
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(Vobj::Double(value1 - value2 as f64))
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }

            Vobj::Int(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(Vobj::Double(value1 as f64 - value2))
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(Vobj::Int(value1 - value2))
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }
            _ => Err(VMError::IncorrectArgumentErr(String::from(
                "Expected numerical type",
            ))),
        }
    }

    pub fn mul(oprand1: &Vobj, oprand2: &Vobj) -> Result<Vobj, VMError> {
        match *oprand1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(Vobj::Double(value1 * value2))
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(Vobj::Double(value1 * value2 as f64))
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }

            Vobj::Int(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(Vobj::Double(value1 as f64 * value2))
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(Vobj::Int(value1 * value2))
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }
            _ => Err(VMError::IncorrectArgumentErr(String::from(
                "Expected numerical type",
            ))),
        }
    }

    pub fn div(oprand1: &Vobj, oprand2: &Vobj) -> Result<Vobj, VMError> {
        match *oprand1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    if value2 == 0.0 {
                        Err(VMError::DivisionByZeroErr)
                    } else {
                        Ok(Vobj::Double(value1 / value2))
                    }
                } else if let Vobj::Int(value2) = *oprand2 {
                    if value2 == 0 {
                        Err(VMError::DivisionByZeroErr)
                    } else {
                        Ok(Vobj::Double(value1 / value2 as f64))
                    }
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }

            Vobj::Int(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    if value2 == 0.0 {
                        Err(VMError::DivisionByZeroErr)
                    } else {
                        Ok(Vobj::Double(value1 as f64 / value2))
                    }
                } else if let Vobj::Int(value2) = *oprand2 {
                    if value2 == 0 {
                        Err(VMError::DivisionByZeroErr)
                    } else {
                        Ok(Vobj::Int(value1 / value2))
                    }
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }
            _ => Err(VMError::IncorrectArgumentErr(String::from(
                "Expected numerical type",
            ))),
        }
    }

    pub fn greater_than(oprand1: &Vobj, oprand2: &Vobj) -> Result<bool, VMError> {
        match *oprand1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(value1>value2)
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(value1>value2 as f64)
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }

            Vobj::Int(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(value1 as f64>value2)
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(value1 >value2)
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }
            _ => Err(VMError::IncorrectArgumentErr(String::from(
                "Expected numerical type",
            ))),
        }
    }

    pub fn greater_eq(oprand1: &Vobj, oprand2: &Vobj) -> Result<bool, VMError> {
        match *oprand1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(value1>=value2)
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(value1>=value2 as f64)
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }

            Vobj::Int(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(value1 as f64>=value2)
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(value1 >=value2)
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }
            _ => Err(VMError::IncorrectArgumentErr(String::from(
                "Expected numerical type",
            ))),
        }
    }

    pub fn less_than(oprand1: &Vobj, oprand2: &Vobj) -> Result<bool, VMError> {
        match *oprand1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(value1<value2)
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(value1<value2 as f64)
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }

            Vobj::Int(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok((value1 as f64) < value2)
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(value1 <value2)
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }
            _ => Err(VMError::IncorrectArgumentErr(String::from(
                "Expected numerical type",
            ))),
        }
    }

    pub fn less_eq(oprand1: &Vobj, oprand2: &Vobj) -> Result<bool, VMError> {
        match *oprand1 {
            Vobj::Double(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(value1<=value2)
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(value1<=value2 as f64)
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }

            Vobj::Int(value1) => {
                if let Vobj::Double(value2) = *oprand2 {
                    Ok(value1 as f64<=value2)
                } else if let Vobj::Int(value2) = *oprand2 {
                    Ok(value1 <=value2)
                } else {
                    Err(VMError::IncorrectArgumentErr(String::from(
                        "Expected numerical type",
                    )))
                }
            }
            _ => Err(VMError::IncorrectArgumentErr(String::from(
                "Expected numerical type",
            ))),
        }
    }

}

impl Default for Vobj {
    fn default() -> Self {
        Vobj::Null
    }
}

impl From<f64> for Vobj {
   fn from(value: f64) -> Self {
       Vobj::Double(value) 
    } 
}

impl From<i64> for Vobj {
   fn from(value: i64) -> Self {
       Vobj::Int(value) 
    } 
}

impl From<&str> for Vobj {
   fn from(value: &str) -> Self {
       Vobj::Str(String::from(value))
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
