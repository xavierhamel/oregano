use crate::error;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Unit {
    Farad,
    Volt,
    Ampere,
    Henry,
    Ohm,
    Second,
    Hertz,
    Degree,
}

impl Unit {
    pub fn to_string(&self) -> String {
        match self {
            Self::Farad => "F".to_string(),
            Self::Volt => "V".to_string(),
            Self::Ampere => "A".to_string(),
            Self::Henry => "H".to_string(),
            Self::Ohm => "Ω".to_string(),
            Self::Second => "s".to_string(),
            Self::Degree => "deg".to_string(),
            Self::Hertz => "Hz".to_string(),
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::Farad => "F",
            Self::Volt => "V",
            Self::Ampere => "A",
            Self::Henry => "H",
            Self::Ohm => "Ω",
            Self::Second => "s",
            Self::Degree => "deg",
            Self::Hertz => "Hz",
        };
        write!(f, "{}", out)
    }
}

impl std::str::FromStr for Unit {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, error::Error> {
        match s {
            "F" => Ok(Self::Farad),
            "V" => Ok(Self::Volt),
            "A" => Ok(Self::Ampere),
            "H" => Ok(Self::Henry),
            "Ω" => Ok(Self::Ohm),
            "s" => Ok(Self::Second),
            "deg" => Ok(Self::Degree),
            "Hz" => Ok(Self::Hertz),
            _ => Err(Box::new(error::Internal::Parse)),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Prefix {
    Giga,
    Mega,
    Kilo,
    None,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
}

impl Prefix {
    pub fn index(&self) -> usize {
        Prefix::as_array().iter().position(|p| p == self).unwrap()
    }

    pub fn as_array() -> [Self; 10] {
        [
            Self::Giga,
            Self::Mega,
            Self::Kilo,
            Self::None,
            Self::Deci,
            Self::Centi,
            Self::Milli,
            Self::Micro,
            Self::Nano,
            Self::Pico,
        ]
    }
}

impl From<usize> for Prefix {
    fn from(value: usize) -> Self {
        Self::as_array()[value]
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::Giga => "G",
            Self::Mega => "M",
            Self::Kilo => "k",
            Self::None => "",
            Self::Deci => "d",
            Self::Centi => "c",
            Self::Milli => "m",
            Self::Micro => "μ",
            Self::Nano => "n",
            Self::Pico => "p",
        };
        write!(f, "{}", out)
    }
}

impl fmt::Debug for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::Giga => "G",
            Self::Mega => "Meg",
            Self::Kilo => "k",
            Self::None => "",
            Self::Deci => "d",
            Self::Centi => "c",
            Self::Milli => "m",
            Self::Micro => "u",
            Self::Nano => "n",
            Self::Pico => "p",
        };
        write!(f, "{}", out)
    }
}

impl std::str::FromStr for Prefix {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, error::Error> {
        match s {
            "G" => Ok(Self::Giga),
            "Meg" => Ok(Self::Mega),
            "k" => Ok(Self::Kilo),
            "" => Ok(Self::None),
            "d" => Ok(Self::Deci),
            "c" => Ok(Self::Centi),
            "m" => Ok(Self::Milli),
            "u" => Ok(Self::Micro),
            "n" => Ok(Self::Nano),
            "p" => Ok(Self::Pico),
            _ => Err(Box::new(error::Internal::Parse)),
        }
    }
}
