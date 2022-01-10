use crate::editor::{component, component::components::Components};

pub fn to_string(component: component::Component) -> String {
    match component.typ {
        Components::SourceVoltageDc => source_voltage_dc_to_string(),
    }
}
