use crate::{dom, project::import, unit};

#[derive(PartialEq, Clone)]
pub enum Property {
    Text(String, bool),
    Num(f64, bool),
    Unit(f64, unit::Prefix, unit::Unit, bool),
    InternalStr(String),
    InternalF64(f64),
}

impl Property {
    pub fn to_string(&self) -> String {
        match self {
            Self::Text(value, _) => value.clone(),
            Self::Num(value, _) => value.to_string(),
            Self::Unit(value, prefix, unit, _) => {
                format!("{} {}{}", value, prefix.to_string(), unit.to_string())
            }
            Self::InternalStr(value) => value.to_string(),
            Self::InternalF64(value) => value.to_string(),
        }
    }

    pub fn to_oregano(&self) -> String {
        match self {
            Self::Text(value, is_visible) => format!("text({};{})", value, is_visible),
            Self::Num(value, is_visible) => format!("num({};{})", value, is_visible),
            Self::Unit(value, prefix, unit, is_visible) => format!(
                "unit({};{};{};{})",
                value,
                prefix.to_spice_sufix(),
                unit.to_string(),
                is_visible
            ),
            Self::InternalStr(value) => format!("internal_str({})", value),
            Self::InternalF64(value) => format!("internal_f64({})", value),
        }
    }

    pub fn from_inputs(prefix: &str) -> Result<Self, ()> {
        let is_visible = match dom::form::checkbox::value_as_bool(&format!(
            "[name=\"{}-is-visible\"]",
            prefix
        )) {
            Ok(value) => value,
            Err(_) => return Err(()),
        };
        let value = match dom::form::text_input::value_as_string(&format!("[name=\"{}\"]", prefix))
        {
            Ok(val) => val,
            Err(_) => return Err(()),
        };
        let unit_value =
            dom::form::text_input::value_as_string(&format!("[name=\"{}-unit\"]", prefix));
        let prefix_value =
            dom::form::select::value_as_usize(&format!("[name=\"{}-unit-prefix\"]", prefix));
        match (unit_value, prefix_value) {
            (Ok(unit_str), Ok(prefix_idx)) => {
                if let Ok(f64_value) = value.parse::<f64>() {
                    let unit = unit::Unit::from_str(&unit_str);
                    Ok(Self::Unit(
                        f64_value,
                        unit::Prefix::as_array()[prefix_idx].clone(),
                        unit,
                        is_visible,
                    ))
                } else {
                    Err(())
                }
            }
            _ => {
                if let Ok(f64_value) = value.parse::<f64>() {
                    Ok(Self::Num(f64_value, is_visible))
                } else {
                    Ok(Self::Text(value, is_visible))
                }
            }
        }
    }

    pub fn from_str(input: &str) -> Result<Self, import::Err> {
        if !input.ends_with(")") {
            return Err(import::Err::MissingToken);
        }
        if let Some(parenthesis_idx) = input.find('(') {
            let values = &input[parenthesis_idx + 1..input.len() - 1]
                .split(";")
                .collect::<Vec<&str>>();
            return match &input[..parenthesis_idx] {
                "text" if values.len() == 2 => {
                    let is_visible = import::utils::parse::<bool>(values[1])?;
                    Ok(Self::Text(values[0].to_string(), is_visible))
                }
                "num" if values.len() == 2 => {
                    let value = import::utils::parse::<f64>(values[0])?;
                    let is_visible = import::utils::parse::<bool>(values[1])?;
                    Ok(Self::Num(value, is_visible))
                }
                "unit" if values.len() == 4 => {
                    let value = import::utils::parse::<f64>(values[0])?;
                    let prefix = unit::Prefix::from_str(values[1]);
                    let unit = unit::Unit::from_str(values[2]);
                    let is_visible = import::utils::parse::<bool>(values[3])?;
                    Ok(Self::Unit(value, prefix, unit, is_visible))
                }
                "internal_str" if values.len() == 1 => Ok(Self::InternalStr(values[0].to_string())),
                "internal_f64" if values.len() == 1 => {
                    let value = import::utils::parse::<f64>(values[0])?;
                    Ok(Self::InternalF64(value))
                }
                "text" | "num" | "unit" | "internal_str" | "internal_f64" => {
                    Err(import::Err::MissingValues)
                }
                _ => Err(import::Err::UnexpectedToken),
            };
        } else {
            Err(import::Err::MissingToken)
        }
    }
}

/// Return some data necessary for a specific property. It return `(title, some help, prefered order)`.
/// Where the order is the prefered position starting at 0.
pub fn metadata_en(key: &str) -> (&'static str, &'static str, usize) {
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
            return ("", "", 100);
        }
    }
}
