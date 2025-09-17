use std::fmt::Display;
use std::fmt::{Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Platform {
    PumpFun, 
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Platform::PumpFun => "PumpFun",
        };
        write!(f, "{s}")
    }
}