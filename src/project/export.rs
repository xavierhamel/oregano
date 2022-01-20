use crate::editor::entity::Entity;

pub fn to_oregano(entities: &Vec<Box<dyn Entity>>) -> String {
    let mut wires = "".to_string();
    let mut components = "".to_string();
    for entity in entities {
        if entity.is_wire() {
            wires.push_str(&entity.to_oregano());
        } else {
            components.push_str(&entity.to_oregano());
        }
    }
    let output = format!(
        "[WIRES]\n{}[COMPONENTS]\n{}[ANALYSIS]\n{}",
        wires, components, "",
    );
    output
}
