use std::{
    error::Error,
    fmt::{self},
};

// TODO - Finish value types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValueType {
    Float(f64),
    Int(i32),
    Byte(u8),
    Nil,
}

impl ValueType {
    pub fn negate(&self) -> Result<Self, Box<dyn Error>> {
        match self {
            ValueType::Float(value) => Ok(ValueType::Float(-value)),
            ValueType::Int(value) => Ok(ValueType::Int(-value)),
            _ => Err("Can't negate non numeric or unsigned values".into()),
        }
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Float(value) => write!(f, "{value:?}"),
            ValueType::Int(value) => write!(f, "{value}"),
            ValueType::Byte(value) => write!(f, "{value}"),
            ValueType::Nil => write!(f, "nil"),
        }
    }
}
