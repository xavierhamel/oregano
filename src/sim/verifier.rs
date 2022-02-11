use crate::error;
use crate::schema::{parts, wires};

pub struct Verifier<'entity> {
    pub errors: Vec<error::Error>,
    parts: &'entity Vec<parts::Part>,
    _wires: &'entity Vec<wires::Wire>,
    names: Vec<String>,
}

impl<'entity> Verifier<'entity> {
    pub fn check(parts: &'entity Vec<parts::Part>, wires: &'entity Vec<wires::Wire>) -> Self {
        let mut verifier = Self {
            errors: Vec::new(),
            parts,
            _wires: wires,
            names: Vec::new(),
        };
        verifier.check_parts();
        verifier
    }

    fn check_parts(&mut self) {
        let mut is_ground_present = false;
        let mut is_node_or_probe_present = false;
        for part in self.parts {
            self.check_for_name_collision(part);
            if &part.typ == "lumped.ground" {
                self.check_ground_name(part);
                is_ground_present = true;
            }
            if &part.typ == "lumped.node" || &part.typ[..5] == "probe" {
                is_node_or_probe_present = true;
            }
        }
        if !is_ground_present {
            self.errors.push(Box::new(error::Sim::NoGround));
        }
        if !is_node_or_probe_present {
            self.errors.push(Box::new(error::Sim::NoProbe));
        }
    }

    fn check_for_name_collision(&mut self, part: &parts::Part) {
        if let Ok(property) = part.properties.get("name") {
            let name = property.value.to_string();
            if self.names.contains(&name) && name != "0" {
                self.errors
                    .push(Box::new(error::Sim::MultipleSameName(name.clone())));
            } else {
                self.names.push(name);
            }
        }
    }

    fn check_ground_name(&mut self, part: &parts::Part) {
        if let Ok(property) = part.properties.get("name") {
            if property.value.to_string() != "0" {
                self.errors.push(Box::new(error::Sim::GroundWithBadName));
            }
        }
    }
}
