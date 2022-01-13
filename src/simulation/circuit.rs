use crate::editor::{component, component::components, entity::Entity, property, wire};
use crate::{dom, simulation};

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
    pub fn new(
        wires: Vec<wire::Wire>,
        components: Vec<component::Component>,
    ) -> Result<Self, simulation::Err> {
        let mut circuit = Self { wires, components };
        let mut is_ground_connected = false;
        for (idx, node) in circuit.nodes().iter().enumerate() {
            let components = circuit.components_connected_to_node(&node);
            let name = circuit.node_name(&components, idx)?;
            for conn in components.iter() {
                if circuit.components[conn.component_idx].typ == components::Components::Ground {
                    is_ground_connected = true;
                }
                circuit.components[conn.component_idx]
                    .connected_to
                    .push((conn.connector_idx, name.clone()));
            }
        }
        if !is_ground_connected {
            Err(simulation::Err::NoGround)
        } else {
            Ok(circuit)
        }
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

    fn node_name(
        &self,
        components: &Vec<Connection>,
        node_idx: usize,
    ) -> Result<String, simulation::Err> {
        let mut names = Vec::new();
        for conn in components.iter() {
            let component = &self.components[conn.component_idx];
            match component.typ {
                components::Components::Ground | components::Components::Node => {
                    if let Some(property::Property::Text(name, _)) =
                        component.properties.get("name")
                    {
                        names.push(name.clone());
                    }
                    if let Some(property::Property::Num(value, _)) =
                        component.properties.get("name")
                    {
                        names.push(value.to_string());
                    }
                }
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

    fn _find_ground(&self) -> Option<usize> {
        for (idx, component) in self.components.iter().enumerate() {
            if component.typ == component::components::Components::Ground {
                return Some(idx);
            }
        }
        None
    }

    fn _wires_connected_to_component(&self, component: &component::Component) -> Vec<usize> {
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
            string.push_str(&simulation::component_to_string(&component)?);
            string.push_str("\n");
        }

        let probes = simulation::probes_to_strings(&self.components)?;
        if probes.len() > 0 {
            let analysis = match dom::form::select::value_as_string("[name=\"sim__type\"]") {
                Ok(typ) if typ == "tran" => simulation::tran_analysis_to_string(probes)?,
                Ok(typ) if typ == "freq" => simulation::freq_analysis_to_string(probes)?,
                _ => return Err(simulation::Err::InvalidAnalysis),
            };
            string.push_str(&analysis);
            Ok(string)
        } else {
            Err(simulation::Err::NoProbeSelected)
        }
    }
}
