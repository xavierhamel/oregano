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

    pub fn from_str(value: &str) -> Self {
        match value {
            "F" => Self::Farad,
            "V" => Self::Volt,
            "A" => Self::Ampere,
            "H" => Self::Henry,
            "Ω" => Self::Ohm,
            "s" => Self::Second,
            "deg" => Self::Degree,
            "Hz" => Self::Hertz,
            _ => Self::Second,
        }
    }
}

#[derive(Clone, PartialEq)]
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
    pub fn to_string(&self) -> String {
        match self {
            Self::Giga => "G".to_string(),
            Self::Mega => "M".to_string(),
            Self::Kilo => "k".to_string(),
            Self::None => "".to_string(),
            Self::Deci => "d".to_string(),
            Self::Centi => "c".to_string(),
            Self::Milli => "m".to_string(),
            Self::Micro => "μ".to_string(),
            Self::Nano => "n".to_string(),
            Self::Pico => "p".to_string(),
        }
    }

    pub fn to_spice_sufix(&self) -> String {
        match self {
            Self::Giga => "G".to_string(),
            Self::Mega => "Meg".to_string(),
            Self::Kilo => "k".to_string(),
            Self::None => "".to_string(),
            Self::Deci => "d".to_string(),
            Self::Centi => "c".to_string(),
            Self::Milli => "m".to_string(),
            Self::Micro => "u".to_string(),
            Self::Nano => "n".to_string(),
            Self::Pico => "p".to_string(),
        }

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
