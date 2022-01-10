mod lumped;
mod source;
pub mod circuit;
pub mod dialog;
use crate::editor::{component, component::components::Components};


pub fn node_name(idx: usize) -> String {
    (0..(idx / 26) + 1).map(|i| {
        ((idx - 26 * i) as u8 + 97u8) as char
    }).collect::<String>()
}

#[derive(Debug)]
pub enum Err {
    /// A component must have a property but it was not found
    PropertyNotFound(&'static str),
    /// A component must have a name longer than 0 char
    EmptyName,
    /// A component must have a specific number of connections (must have, found)
    MissingConnection(usize, usize),
    /// Multiple name or ground are on a single node
    MultipleNameOnNode(usize),
    /// The component is not implement for simulation yet
    NotImplemented,
}

pub fn to_string(component: &component::Component) -> Result<String, Err> {
    match component.typ {
        Components::SourceVoltageDc => source::voltage_dc_to_string(component),
        Components::SourceCurrentDc => source::current_dc_to_string(component),
        Components::SourceVoltageAc => source::voltage_ac_to_string(component),
        Components::SourceCurrentAc => source::current_ac_to_string(component),

        Components::Inductor => lumped::inductor_to_string(component),
        Components::Resistor => lumped::resistor_to_string(component),
        Components::Capacitor => lumped::capacitor_to_string(component),
        Components::Ground | Components::Node => Ok("".to_string()),

        Components::Voltmeter => Ok("".to_string()),
    }
}
