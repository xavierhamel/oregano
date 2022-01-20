use crate::error;
use crate::schema::parts::part;

pub fn lumped_to_string(typ: &str, part: &part::Part) -> Result<String, error::Error> {
    let name = part.properties.get("name")?.value.to_string();
    if name.len() == 0 {
        return Err(Box::new(error::Sim::EmptyName));
    }
    let value = part.properties.get("value")?.value.to_string();
    let connectors = part.connectors()?;

    let init_cond = match part.properties.get("initial_condition") {
        Ok(property) => format!("ic={}", property.value),
        _ => String::new(),
    };

    Ok(format!(
        "{}{} {} {} {} {}",
        typ, name, connectors[0], connectors[1], value, init_cond,
    ))
}

pub fn inductor_to_string(part: &part::Part) -> Result<String, error::Error> {
    lumped_to_string("L", part)
}

pub fn resistor_to_string(part: &part::Part) -> Result<String, error::Error> {
    lumped_to_string("R", part)
}

pub fn capacitor_to_string(part: &part::Part) -> Result<String, error::Error> {
    lumped_to_string("C", part)
}
