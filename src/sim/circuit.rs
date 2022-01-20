use crate::schema::{parts, parts::part, properties, wire};
use crate::{clog, dom, error, sim};

#[derive(PartialEq)]
pub struct Connection {
    pub connector: usize,
    part: usize,
}

impl Connection {
    pub fn new(part: usize, connector: usize) -> Self {
        Self { part, connector }
    }
}

pub struct Circuit {
    wires: Vec<wire::Wire>,
    parts: Vec<part::Part>,
}

impl Circuit {
    pub fn new(wires: Vec<wire::Wire>, parts: Vec<part::Part>) -> Result<Self, error::Error> {
        let mut circuit = Self { wires, parts };
        let mut is_ground_connected = false;
        clog!("{}", circuit.nodes().len());
        circuit.nodes().iter().enumerate().for_each(|(idx, node)| {
            let parts = circuit.parts_connected_to_node(&node);
            clog!("{}", parts.len());
            let name = circuit.node_name(&parts, idx).unwrap();
            parts.iter().for_each(|conn| {
                if circuit.parts[conn.part].typ == parts::Typ::Ground {
                    is_ground_connected = true;
                }
                circuit.parts[conn.part].layout.connect(conn, &name);
            });
        });
        if !is_ground_connected {
            Err(Box::new(error::Sim::NoGround))
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
        connections: &Vec<Connection>,
        node_idx: usize,
    ) -> Result<String, error::Error> {
        let mut names = Vec::new();
        for conn in connections.iter() {
            let part = &self.parts[conn.part];
            if part.typ == parts::Typ::Ground || part.typ == parts::Typ::Node {
                if let Ok(property) = part.properties.get("name") {
                    match &property.value {
                        properties::Value::String(name) => names.push(name.clone()),
                        properties::Value::F64(name) => names.push(name.to_string()),
                        _ => {}
                    }
                }
            }
        }
        if names.len() > 1 {
            Err(Box::new(error::Sim::MultipleNameOnNode(node_idx)))
        } else if names.len() == 1 {
            Ok(names[0].clone())
        } else {
            Ok(sim::node_name(node_idx))
        }
    }

    /// Find all the connected wires to a given one. All connected wires represent a single node.
    ///
    /// TODO: If 2 wires cross over each other, they are currently not returned as connected. Only
    /// wires that the extremities are touching
    fn wires_connected_to_wire(&self, wire: &wire::Wire) -> Vec<usize> {
        self.wires
            .iter()
            .enumerate()
            .filter(|(_, other_wire)| wire.collide_with_wire(other_wire))
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>()
    }

    fn parts_connected_to_node(&self, node: &Vec<wire::Wire>) -> Vec<Connection> {
        node.iter().fold(Vec::new(), |mut acc, wire| {
            acc.append(
                &mut self
                    .parts_connected_to_wire(wire)
                    .into_iter()
                    .filter(|connection| !acc.contains(connection))
                    .collect::<Vec<Connection>>(),
            );
            acc
        })
    }

    fn parts_connected_to_wire(&self, wire: &wire::Wire) -> Vec<Connection> {
        self.parts
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (idx, part)| {
                acc.append(
                    &mut wire
                        .collide_with_part(part)
                        .iter()
                        .map(|conn_idx| Connection::new(idx, *conn_idx))
                        .collect::<Vec<Connection>>(),
                );
                acc
            })
    }

    pub fn to_string(&self) -> Result<String, error::Error> {
        let mut string = String::from("A description of the circuit\n");
        for part in self.parts.iter() {
            string.push_str(&sim::part_to_string(&part)?);
            string.push_str("\n");
        }

        let probes = sim::probes_to_strings(&self.parts)?;
        if probes.len() > 0 {
            let analysis =
                match dom::form::select::value::<String>(dom::select("[name=\"sim__type\"]")) {
                    Ok(typ) if typ == "tran" => sim::tran_analysis_to_string(probes)?,
                    Ok(typ) if typ == "freq" => sim::freq_analysis_to_string(probes)?,
                    err => return err,
                };
            string.push_str(&analysis);
            Ok(string)
        } else {
            Err(Box::new(error::Sim::NoProbe))
        }
    }

    // fn _find_ground(&self) -> Option<usize> {
    //     for (idx, component) in self.components.iter().enumerate() {
    //         if component.typ == component::components::Components::Ground {
    //             return Some(idx);
    //         }
    //     }
    //     None
    // }

    // fn _wires_connected_to_component(&self, component: &component::Component) -> Vec<usize> {
    //     let mut connected = vec![];
    //     for (idx, wire) in self.wires.iter().enumerate() {
    //         'outer: for &contact in &wire.shape().polygones[0] {
    //             for conn in component.connections().iter() {
    //                 if contact == *conn {
    //                     connected.push(idx);
    //                     break 'outer;
    //                 }
    //             }
    //         }
    //     }
    //     connected
    // }
}
