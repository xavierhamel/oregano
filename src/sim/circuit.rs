use crate::schema::{parts, properties, wires};
use crate::sim::verifier;
use crate::{dom, dom::form::select, error, sim, PARTS};
use std::convert::TryFrom;

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
    wires: Vec<wires::Wire>,
    parts: Vec<parts::Part>,
    errors: Vec<error::Error>,
}

impl Circuit {
    pub fn new(
        wires: Vec<wires::Wire>,
        parts: Vec<parts::Part>,
    ) -> Result<Self, Vec<error::Error>> {
        let mut circuit = Self {
            wires,
            parts,
            errors: Vec::new(),
        };
        let mut verifier = verifier::Verifier::check(&circuit.parts, &circuit.wires);
        circuit.errors.append(&mut verifier.errors);
        circuit.pad_parts_connectors();
        circuit.connect_parts_to_node();
        if circuit.errors.len() == 0 {
            Ok(circuit)
        } else {
            Err(circuit.errors)
        }
    }

    /// Here we add wires where the connectors are. This is done because when there is no wires
    /// between two parts, those parts are still connected. Adding those wire connect the parts
    /// together. If no wires were added, the parts would not be seen as connected.
    fn pad_parts_connectors(&mut self) {
        for part in self.parts.iter() {
            for connector in part.layout.connectors.iter() {
                self.wires.push(wires::Wire::new(
                    part.layout.origin + connector.origin,
                    part.layout.origin + connector.origin,
                ));
            }
        }
    }

    /// Connect every part to a name node. Also check if a node is missing a connection.
    fn connect_parts_to_node(&mut self) {
        self.nodes().iter().enumerate().for_each(|(idx, node)| {
            let parts = self.parts_connected_to_node(&node);
            // TODO: Add a check for name collision with named node. If a node is name `a`, do not
            // give the node the generic name `a`.
            let name = self.node_name(&parts, idx).unwrap();
            if parts.len() < 2 {
                self.errors
                    .push(Box::new(error::Sim::MissingConnectionNode(name.clone())))
            }
            parts.iter().for_each(|conn| {
                self.parts[conn.part].layout.connect(conn, &name);
            });
        });
    }

    fn nodes(&mut self) -> Vec<Vec<wires::Wire>> {
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

    fn find_wires_for_node(&self, parent: &wires::Wire) -> Vec<usize> {
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
            if part.typ == "lumped.ground".to_string() || part.typ == "lumped.node".to_string() {
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
    fn wires_connected_to_wire(&self, wire: &wires::Wire) -> Vec<usize> {
        self.wires
            .iter()
            .enumerate()
            .filter(|(_, other_wire)| wire.collide_with_wire(other_wire))
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>()
    }

    fn parts_connected_to_node(&self, node: &Vec<wires::Wire>) -> Vec<Connection> {
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

    fn parts_connected_to_wire(&self, wire: &wires::Wire) -> Vec<Connection> {
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

    pub fn to_string(&self) -> Result<(String, sim::Probes), error::Error> {
        let mut parts = String::from("A Circuit\n");
        for part in self.parts.iter() {
            parts.push_str(&part.to_spice()?);
            parts.push_str("\n");
        }

        let probes = sim::Probes::try_from(&self.parts)?;
        let simulation = select::value::<String>(dom::select("[name=\"sim__type\"]"))?;
        let analysis = sim::Analysis::try_from((simulation, &probes))?;
        Ok((format!("{}{}{}", parts, PARTS.models(), analysis), probes))
    }
}
