pub mod circuit;
pub mod dialog;
mod lumped;
mod source;
use crate::editor::{component, component::components::Components, entity::Entity, property};
use crate::{dom, unit};
use wasm_bindgen::JsCast;

pub fn node_name(idx: usize) -> String {
    (0..(idx / 26) + 1)
        .map(|i| ((idx - 26 * i) as u8 + 97u8) as char)
        .collect::<String>()
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
    /// No probe were selected for the simulation
    NoProbeSelected,
    /// The selected analysis does not exist
    InvalidAnalysis,
    /// There was no ground in the circuit (this should maybe be a warning?)
    NoGround,
}

impl Err {
    pub fn show(&self) {
        let message = match self {
            Err::PropertyNotFound(name) => {
                format!("The property named \"{}\" was not found in the list of properties of a component.", name)
            }
            Err::EmptyName => {
                format!("A component cannot have an empty name. Please give a name of at least one character to each component.")
            }
            Err::MissingConnection(_, _) => {
                format!("A component is missing at least one connection. Please connect every connections of every components")
            }
            Err::MultipleNameOnNode(_) => {
                format!("Each node can only have one name or a ground. One of the node (contiguous wire) as multiple name or multiple ground")
            }
            Err::NoProbeSelected => {
                format!("You have to select at least one probe to execute a simulation")
            }
            Err::InvalidAnalysis => {
                format!(
                    "The selected analysis does not exists. Please select an other analysis type."
                )
            }
            Err::NoGround => {
                format!(
                    "No ground were connected to the circuit. Please add a ground to one of the nodes in the circuit"
                )
            }
        };
        dom::select("#error__message").set_inner_html(&message);
        dom::select("#error__container")
            .set_attribute("class", "")
            .unwrap();
    }
}

pub fn component_to_string(component: &component::Component) -> Result<String, Err> {
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

pub fn probes_to_strings(components: &Vec<component::Component>) -> Result<Vec<String>, Err> {
    let mut probes = Vec::new();
    for component in components {
        if let Some(Components::Voltmeter) = component.typ() {
            let nodes = if component.connected_to[0].0 == 0 {
                (
                    component.connected_to[0].1.clone(),
                    component.connected_to[1].1.clone(),
                )
            } else {
                (
                    component.connected_to[1].1.clone(),
                    component.connected_to[0].1.clone(),
                )
            };
            probes.push(format!("({},{})", nodes.0, nodes.1));
        }
    }
    Ok(probes)
}

pub fn tran_analysis_to_string(probes: Vec<String>) -> Result<String, Err> {
    let probes = probes.iter().fold("".to_string(), |mut string, probe| {
        string.push_str(" v");
        string.push_str(probe);
        string
    });
    Ok(format!(
        "{}\n.tran {}{} {}{} uic\n.print tran{}\n.end",
        spice_options(),
        dom::form::text_input::value_as_string("[name=\"tran__step\"]").unwrap(),
        unit::Prefix::as_array()
            [dom::form::select::value_as_usize("[name=\"tran__step-prefix\"]").unwrap()]
        .to_spice_sufix(),
        dom::form::text_input::value_as_string("[name=\"tran__stop\"]").unwrap(),
        unit::Prefix::as_array()
            [dom::form::select::value_as_usize("[name=\"tran__stop-prefix\"]").unwrap()]
        .to_spice_sufix(),
        probes
    ))
}

pub fn freq_analysis_to_string(probes: Vec<String>) -> Result<String, Err> {
    let data_type = dom::form::select::value_as_string("[name=\"sim__freq-data-type\"]").unwrap();
    let probes = probes.iter().fold("".to_string(), |mut string, probe| {
        string.push_str(&format!(" v{}", data_type));
        string.push_str(probe);
        string
    });
    Ok(format!(
        "{}\n.ac {} {} {}{} {}{}\n.print ac{}\n.end",
        spice_options(),
        dom::form::select::value_as_string("[name=\"sim__freq-variation-type\"]").unwrap(),
        dom::form::text_input::value_as_string("[name=\"sim__freq-np\"]").unwrap(),
        dom::form::text_input::value_as_string("[name=\"sim__freq-fstart\"]").unwrap(),
        unit::Prefix::as_array()
            [dom::form::select::value_as_usize("[name=\"sim__freq-fstart-prefix\"]").unwrap()]
        .to_spice_sufix(),
        dom::form::text_input::value_as_string("[name=\"sim__freq-fstop\"]").unwrap(),
        unit::Prefix::as_array()
            [dom::form::select::value_as_usize("[name=\"sim__freq-fstop-prefix\"]").unwrap()]
        .to_spice_sufix(),
        probes
    ))
}

fn spice_options() -> &'static str {
    ".options NOACCT"
}
