use crate::error;
use crate::schema::parts::part;

pub fn dc_to_string(typ: &str, part: &part::Part) -> Result<String, error::Error> {
    let name = part.properties.get("name")?.value.to_string();
    if name.len() == 0 {
        return Err(Box::new(error::Sim::EmptyName));
    }
    let value = part.properties.get("value")?.value.to_string();
    let connectors = part.connectors()?;

    Ok(format!(
        "{}{} {} {} dc {}",
        typ, name, connectors[0], connectors[1], value,
    ))
}

pub fn current_dc_to_string(part: &part::Part) -> Result<String, error::Error> {
    dc_to_string("I", part)
}

pub fn voltage_dc_to_string(part: &part::Part) -> Result<String, error::Error> {
    dc_to_string("V", part)
}

pub fn ac_to_string(typ: &str, part: &part::Part) -> Result<String, error::Error> {
    let name = part.properties.get("name")?.value.to_string();
    if name.len() == 0 {
        return Err(Box::new(error::Sim::EmptyName));
    }
    let connectors = part.connectors()?;

    let properties = vec![
        "offset",
        "amplitude",
        "frequency",
        "delay",
        "damping_factor",
        "phase",
    ]
    .iter()
    .map(|key| match part.properties.get(key) {
        Ok(property) => Ok(property.value.to_string()),
        Err(err) => Err(err),
    })
    .collect::<Result<Vec<String>, error::Error>>()?;

    Ok(format!(
        "{}{} {} {} ac 1 SIN({} {} {} {} {} {})",
        typ,
        name,
        connectors[0],
        connectors[1],
        properties[0],
        properties[1],
        properties[2],
        properties[3],
        properties[4],
        properties[5],
    ))
}

pub fn current_ac_to_string(part: &part::Part) -> Result<String, error::Error> {
    ac_to_string("I", part)
}

pub fn voltage_ac_to_string(part: &part::Part) -> Result<String, error::Error> {
    ac_to_string("V", part)
}
