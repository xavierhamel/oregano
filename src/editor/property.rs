use crate::{unit, dom, log};

#[derive(PartialEq, Clone)]
pub enum Property {
    Text(String, bool),
    Num(f64, bool),
    Unit(f64, unit::Prefix, unit::Unit, bool),
    InternalStr(&'static str),
    InternalF64(f64),
}

impl Property {
    pub fn to_string(&self) -> String {
        match self {
            Self::Text(value, _) => value.clone(),
            Self::Num(value, _) => value.to_string(),
            Self::Unit(value, prefix, unit, _) => format!("{} {}{}", value, prefix.to_string(), unit.to_string()),
            Self::InternalStr(value) => value.to_string(),
            Self::InternalF64(value) => value.to_string(),
        }
    }

    pub fn from_inputs(prefix: &str) -> Result<Self, ()> {
        let is_visible = match dom::form::checkbox::value_as_bool(&format!("[name=\"{}-is-visible\"]", prefix)) {
            Ok(value) => value,
            Err(_) => { return Err(()) },
        };
        let value = match dom::form::text_input::value_as_string(&format!("[name=\"{}\"]", prefix)) {
            Ok(val) => val,
            Err(_) => { return Err(()) },
        };
        let unit_value = dom::form::text_input::value_as_string(&format!("[name=\"{}-unit\"]", prefix));
        let prefix_value = dom::form::select::value_as_usize(&format!("[name=\"{}-unit-prefix\"]", prefix));
        match (unit_value, prefix_value) {
            (Ok(unit_str), Ok(prefix_idx)) => {
                if let Ok(f64_value) = value.parse::<f64>() {
                    let unit = unit::Unit::from_str(&unit_str);
                    Ok(Self::Unit(f64_value, unit::Prefix::as_array()[prefix_idx].clone(), unit, is_visible))
                } else {
                    Err(())
                }
            },
            _ => {
                if let Ok(f64_value) = value.parse::<f64>() {
                    Ok(Self::Num(f64_value, is_visible))
                } else {
                    Ok(Self::Text(value, is_visible))
                }
            }
        }
    }
}


/// Return some data necessary for a specific property. It return `(title, some help, prefered order)`.
/// Where the order is the prefered position starting at 0.
pub fn metadata_en(key: &'static str) -> (&'static str, &'static str, usize) {
    match key {
        // General
        "name" => ("Name", "The name of the component.", 0),
        "value" => ("Value", "The value of the component with it's units.", 1),
        "initial_condition" => ("Initial Condition", "The initial condition is used when a transient analysis is desired to start from other than the quiescent operating point.", 2),
        // AC source
        "amplitude" => ("Amplitude", "Amplitude is the maximum value of current or voltage.", 20),
        "offset" => ("Offset", "", 21),
        "frequency" => ("Frequency", "Frequency of the AC signal", 22),
        "delay" => ("Delay", "", 23),
        "damping_factor" => ("Damping Factor", "", 24),
        "phase" => ("Phase", "", 25),
        _ => {
            log(&format!("A property of a component does not have it's metadata filled: {}", key));
            return (key, key, 100);
        }
    }
}
