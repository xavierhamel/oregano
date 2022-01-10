use crate::editor::{component, entity::Property};
use crate::simulation;

pub fn lumped_to_string(typ: &str, component: &component::Component) -> Result<String, simulation::Err> {
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

    let init_cond = if let Some(Property::Unit(value, prefix, _, _)) = component.properties.get("initial_condition") {
        format!("ic={}{}", value, prefix.to_string())
    } else {
        "".to_string()
    };
    
    Ok(
        format!(
            "{}{} {} {} {} {}",
            typ,
            name,
            component.connected_to[0].1,
            component.connected_to[1].1,
            value,
            init_cond,
        )
    )
}

pub fn inductor_to_string(component: &component::Component) -> Result<String, simulation::Err> {
    lumped_to_string("L", component)
}

pub fn resistor_to_string(component: &component::Component) -> Result<String, simulation::Err> {
    lumped_to_string("R", component)
}

pub fn capacitor_to_string(component: &component::Component) -> Result<String, simulation::Err> {
    lumped_to_string("C", component)
}
