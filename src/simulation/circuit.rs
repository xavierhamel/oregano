use crate::editor::{
    wire,
    component,
    component::components,
    entity::Entity,
    entity
};
use crate::{simulation, dom, unit};


#[derive(PartialEq)]
pub struct Connection {
    connector_idx: usize,
    component_idx: usize,
}

pub struct Circuit {
    wires: Vec<wire::Wire>,
    components: Vec<component::Component>,
}

impl Circuit {
    pub fn new(wires: Vec<wire::Wire>, components: Vec<component::Component>) -> Result<Self, simulation::Err> {
        let mut circuit = Self {
            wires,
            components,
        };
        for (idx, node) in circuit.nodes().iter().enumerate() {
            let components = circuit.components_connected_to_node(&node);
            let name = circuit.node_name(&components, idx)?;
            for conn in components.iter() {
                circuit.components[conn.component_idx].connected_to.push(
                    (conn.connector_idx, name.clone())
                );
            }
        }
        Ok(circuit)
    }

    fn nodes(&mut self) -> Vec<Vec<wire::Wire>> {
        let mut nodes = Vec::new();
        let mut visited = Vec::new();
        for (idx, wire) in self.wires.iter().enumerate() {
            if !visited.contains(&idx) {
                let mut node = vec![wire.clone()];
                let other_wires = self.find_wires_for_node(&wire);
                visited.append(&mut other_wires.clone());
                for &other_wire in other_wires.iter() {
                    node.push(self.wires[other_wire].clone());
                }
                nodes.push(node);
            }
        }
        nodes
    }

    fn find_wires_for_node(&self, parent: &wire::Wire) -> Vec<usize> {
        let mut new_wires_idx = self.wires_connected_to_wire(parent);
        let mut connected = new_wires_idx.clone();
        
        while new_wires_idx.len() > 0 {
            let mut new_wires = Vec::new();
            for &wire_idx in new_wires_idx.iter() {
                let wires = self.wires_connected_to_wire(&self.wires[wire_idx]);
                for &next_wire_idx in wires.iter() {
                    if !connected.contains(&next_wire_idx) {
                        new_wires.push(next_wire_idx);
                        connected.push(next_wire_idx);
                    }
                }
            }
            new_wires_idx = new_wires;
        }
        connected
    }

    fn node_name(&self, components: &Vec<Connection>, node_idx: usize) -> Result<String, simulation::Err> {
        let mut names = Vec::new();
        for conn in components.iter() {
            let component = &self.components[conn.component_idx];
            match component.typ {
                components::Components::Ground | components::Components::Node => {
                    if let Some(entity::Property::Text(name, _)) = component.properties.get("name") {
                        names.push(name.clone());
                    }
                },
                _ => {}
            };
        }
        if names.len() > 1 {
            Err(simulation::Err::MultipleNameOnNode(node_idx))
        } else if names.len() == 1 {
            Ok(names[0].clone())
        } else {
            Ok(simulation::node_name(node_idx))
        }

    }

    /// Find all the connected wires to a given one. All connected wires represent a single node.
    ///
    /// TODO: If 2 wires cross over each other, they are currently not returned as connected. Only
    /// wires that the extremities are touching
    fn wires_connected_to_wire(&self, wire: &wire::Wire) -> Vec<usize> {
        let mut connected = vec![];
        for (idx, other_wire) in self.wires.iter().enumerate() {
            if other_wire != wire {
                for other_contact in &other_wire.shape().polygones[0] {
                    if wire.collide_with_point(*other_contact + other_wire.origin()) {
                        connected.push(idx);
                        break;
                    }
                }
            }
        }
        connected
    }

    fn components_connected_to_node(&self, node: &Vec<wire::Wire>) -> Vec<Connection> {
        let mut components = Vec::new();
        for wire in node.iter() {
            for conn in self.components_connected_to_wire(&wire) {
                if !components.contains(&conn) {
                    components.push(conn);
                }
            }
        }
        components
    }

    fn components_connected_to_wire(&self, wire: &wire::Wire) -> Vec<Connection> {
        let mut connections = Vec::new();
        for (idx, component) in self.components.iter().enumerate() {
            for (conn_idx, conn) in component.connections().iter().enumerate() {
                if wire.collide_with_point(*conn + component.origin()) {
                    connections.push(Connection {
                        component_idx: idx,
                        connector_idx: conn_idx,
                    });
                }
            }
        }
        connections
    }

    fn find_ground(&self) -> Option<usize> {
        for (idx, component) in self.components.iter().enumerate() {
            if component.typ == component::components::Components::Ground {
                return Some(idx)
            }
        }
        None
    }

    fn wires_connected_to_component(&self, component: &component::Component) -> Vec<usize> {
        let mut connected = vec![];
        for (idx, wire) in self.wires.iter().enumerate() {
            'outer: for &contact in &wire.shape().polygones[0] {
                for conn in component.connections().iter() {
                    if contact == *conn {
                        connected.push(idx);
                        break 'outer;
                    }
                }
            }
        }
        connected
    }

    pub fn to_string(&self) -> Result<String, simulation::Err> {
        let mut string = String::from("A description of the circuit\n");
        for component in self.components.iter() {
            string.push_str(&simulation::to_string(&component)?);
            string.push_str("\n");
        }
        // We find the selected probe, with that we will be able to insert the correct value to
        // print.
        let mut nodes = None;
        if let Ok(selected_probe) = dom::form::select::value_as_string("[name=\"tran__node\"]") {
            for component in &self.components {
                if let Some(components::Components::Voltmeter) = component.typ() {
                    if let Some(entity::Property::Text(name, _)) = component.properties.get("name") {
                        if name == &selected_probe {
                            nodes = if component.connected_to[0].0 == 0 {
                                Some((component.connected_to[0].1.clone(), component.connected_to[1].1.clone()))
                            } else {
                                Some((component.connected_to[1].1.clone(), component.connected_to[0].1.clone()))
                            };
                            break;
                        }
                    }
                }
            }
        }
        if let Some((positive, negative)) = nodes {
            string.push_str(
                &format!(
                    "{}\n.tran {}{} {}{} uic\n.print tran v({},{})\n.end",
                    Circuit::spice_options(),
                    dom::form::text_input::value_as_string("[name=\"tran__step\"]").unwrap(),
                    unit::Prefix::as_array()[
                        dom::form::select::value_as_usize("[name=\"tran__step-prefix\"]").unwrap()
                    ].to_spice_sufix(),
                    dom::form::text_input::value_as_string("[name=\"tran__stop\"]").unwrap(),
                    unit::Prefix::as_array()[
                        dom::form::select::value_as_usize("[name=\"tran__stop-prefix\"]").unwrap()
                    ].to_spice_sufix(),
                    positive,
                    negative
                )
            );
            Ok(string)
        } else {
            Err(simulation::Err::MissingConnection(2, 0))
        }
    }

    fn spice_options() -> &'static str {
        ".options NOACCT"
    }
}
