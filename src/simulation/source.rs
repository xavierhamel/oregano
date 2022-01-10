use crate::editor::{component, entity::Property};
use crate::simulation;

pub fn dc_to_string(typ: &str, component: &component::Component) -> Result<String, simulation::Err> {
    let name = if let Some(Property::Text(name, _)) = component.properties.get("name") {
        if name.len() == 0 {
            return Err(simulation::Err::EmptyName);
        }
        name
    } else {
        return Err(simulation::Err::PropertyNotFound("name"));
    };

    if component.connected_to.len() != 2 {
        return Err(simulation::Err::MissingConnection(2, component.connected_to.len()));
    }

    let value = if let Some(Property::Unit(value, prefix, _, _)) = component.properties.get("value") {
        format!("{}{}", value, prefix.to_spice_sufix())
    } else {
        return Err(simulation::Err::PropertyNotFound("value"));
    };
    
    let (positive, negative) = if component.connected_to[0].0 == 0 {
        (component.connected_to[0].1.clone(), component.connected_to[1].1.clone())
    } else {
        (component.connected_to[1].1.clone(), component.connected_to[0].1.clone())
    };

    Ok(
        format!(
            "{}{} {} {} dc {}",
            typ,
            name,
            positive,
            negative,
            value,
        )
    )
}

pub fn current_dc_to_string(component: &component::Component) -> Result<String, simulation::Err> {
    dc_to_string("I", component)
}

pub fn voltage_dc_to_string(component: &component::Component) -> Result<String, simulation::Err> {
    dc_to_string("V", component)
}

pub fn ac_to_string(typ: &str, component: &component::Component) -> Result<String, simulation::Err> {
    let name = if let Some(Property::Text(name, _)) = component.properties.get("name") {
        if name.len() == 0 {
            return Err(simulation::Err::EmptyName);
        }
        name
    } else {
        return Err(simulation::Err::PropertyNotFound("name"));
    };

    if component.connected_to.len() != 2 {
        return Err(simulation::Err::MissingConnection(2, component.connected_to.len()));
    }

    let properties = vec!["offset", "amplitude", "frequency", "delay", "damping_factor", "phase"]
        .iter()
        .map(|property| {
            if let Some(Property::Unit(value, prefix, _, _)) = component.properties.get(property) {
                return Ok(format!("{}{}", value, prefix.to_spice_sufix()));
            } else {
                return Err(simulation::Err::PropertyNotFound("value"));
            }
        })
        .collect::<Result<Vec<String>, simulation::Err>>()?;

    let (positive, negative) = if component.connected_to[0].0 == 0 {
        (component.connected_to[0].1.clone(), component.connected_to[1].1.clone())
    } else {
        (component.connected_to[1].1.clone(), component.connected_to[0].1.clone())
    };

    Ok(
        format!(
            "{}{} {} {} ac 1 SIN({} {} {} {} {} {})",
            typ,
            name,
            positive,
            negative,
            properties[0],
            properties[1],
            properties[2],
            properties[3],
            properties[4],
            properties[5],
        )
    )
}

pub fn current_ac_to_string(component: &component::Component) -> Result<String, simulation::Err> {
    ac_to_string("I", component)
}

pub fn voltage_ac_to_string(component: &component::Component) -> Result<String, simulation::Err> {
    ac_to_string("V", component)
}
