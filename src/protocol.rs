use crate::imports::*;
use crate::result::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Krc20,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Protocol::Krc20 => write!(f, "KRC-20"),
        }
    }
}

impl FromStr for Protocol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "KRC-20" => Ok(Protocol::Krc20),
            _ => Err(Error::Custom(format!("Invalid protocol: {}", s))),
        }
    }
}
