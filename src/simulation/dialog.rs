use crate::editor::{component::components, entity};
use crate::dom;

pub fn update_probes(entities: &Vec<Box<dyn entity::Entity>>) {
    let mut options = Vec::new();
    for entity in entities {
        if let Some(components::Components::Voltmeter) = entity.typ() {
            if let Some(entity::Property::Text(name, _)) = entity.properties().get("name") {
                options.push((name.clone(), name.clone()));
            }
        }
    }
    dom::form::select::update_options("tran__node", options);
}
