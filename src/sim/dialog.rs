//use crate::dom;
//use crate::editor::{component::components, entity, property};

// pub fn update_probes(entities: &Vec<Box<dyn entity::Entity>>) {
//     dom::select("#menu__simulations-probes").set_inner_html("");
//     for entity in entities {
//         if let Some(components::Components::Voltmeter) = entity.typ() {
//             if let Some(property::Property::Text(name, _)) = entity.properties().get("name") {
//                 dom::append_children(
//                     &dom::select("#menu__simulations-probes"),
//                     vec![&dom::form::group(vec![
//                         dom::create_element(
//                             "input",
//                             dom::attributes! {
//                                 "type" => "checkbox",
//                                 "name" => "tran__probes",
//                                 "value" => name,
//                             },
//                             vec![],
//                         ),
//                         dom::form::label::create(&format!("{} (Voltmeter)", name)),
//                     ])],
//                 );
//             }
//         }
//     }
// }
