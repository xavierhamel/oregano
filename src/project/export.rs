use crate::schema::{parts, wires};

pub fn to_oregano(wires: &Vec<wires::Wire>, parts: &Vec<parts::Part>) -> String {
    let wires_section = wires
        .iter()
        .fold(String::from("[WIRES]:::"), |mut acc, wire| {
            acc.push_str(&format!("{:?}:::", wire));
            acc
        });
    let parts_section = parts
        .iter()
        .fold(String::from("[PARTS]:::"), |mut acc, part| {
            acc.push_str(&format!("{:?}:::", part));
            acc
        });
    format!("{}{}[ANALYSIS]:::{}", wires_section, parts_section, "")
}
