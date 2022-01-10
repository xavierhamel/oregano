use crate::dom;
use crate::unit;

pub fn generate_toolbar() {
    let tools = dom::create_element(
        "div",
        dom::attributes!{ "class" => "flex-row" },
        dom::form::radio::create_from_vec(
            "toolbar__mouse",
            vec!["mouse", "wire"],
            vec![
                "<i class=\"fas fa-mouse-pointer\"></i>",
                "<i class=\"fas fa-draw-polygon\"></i>"
            ]
        )
    );
    dom::append_children(
        &dom::select("#menu__toolbar-container"),
        vec![
            &dom::create_element(
                "button",
                dom::attributes!{ 
                    "class" => "form__button-square",
                    "inner_html" => "<i class=\"fas fa-save\"></i>",
                },
                vec![]
            ),
            &dom::create_element(
                "button",
                dom::attributes!{
                    "class" => "form__button-square",
                    "inner_html" => "<i class=\"fas fa-file-download\"></i>"
                },
                vec![]
            ),
            &dom::create_element("div", dom::attributes!{ "class" => "toolbar__separator" }, vec![]),
            &tools,
            &dom::create_element("div", dom::attributes!{ "class" => "toolbar__separator" }, vec![]),
            &dom::form::checkbox::create("toolbar__components", true, "<i class=\"fas fa-bolt\"></i>"),
            &dom::form::checkbox::create("toolbar__properties", true, "<i class=\"fas fa-align-justify\"></i>"),
            &dom::form::checkbox::create("toolbar__simulation", true, "<i class=\"fas fa-cog\"></i>"),
        ]
    );
}

pub fn generate_simulation() {
    dom::append_children(
        &dom::select("#menu__simulations"),
        vec![
            &dom::form::label::create("Type de simulation"),
            &dom::form::select::create("sim__type", vec![("tran".to_string(), "Transitoire".to_string())], 0),
            &dom::form::label::create("Node (simulation)"),
            &dom::form::select::create("tran__node", Vec::new(), 0),
            &dom::form::label::create("Step"),
            &dom::form::group(vec![
                dom::form::text_input::create("tran__step", "1"),
                dom::form::select::create_unit("tran__step-prefix", &unit::Unit::Second, 3),
            ]),
            &dom::form::label::create("Stop"),
            &dom::form::group(vec![
                dom::form::text_input::create("tran__stop", "100"),
                dom::form::select::create_unit("tran__stop-prefix", &unit::Unit::Second, 3),
            ]),
        ]
    );
}

// pub fn generate() {
//     dom::append_children(
//         &dom::select("#container"),
//         vec![
//             &dom::create_element(
//                 "div",
//                 dom::attributes!{ "class" => "flex-row view__container" },
//                 vec![
//                     dom::panel::create(
//                         "panel__components",
//                         "view__components-list",
//                         vec![
//                             (
//                                 "Components".to_string(),
//                                 vec![
//                                     dom::create_element(
//                                         "div",
//                                         dom::attributes!{
//                                             "id" => "components-list__container",
//                                             "class" => "components-container scrollable",
//                                         },
//                                         vec![],
//                                     ),
//                                 ],
//                             )
//                         ]
//                     ),
//                     dom::create_element(
//                         "div",
//                         dom::attributes!{
//                             "class" => "flex-col view__editor-container"
//                         },
//                         vec![
//                             dom::create_element(
//                                 "div",
//                                 dom::attributes!{
//                                     "class" => "view__editor"
//                                 },
//                                 vec![
//                                     dom::create_element(
//                                         "canvas",
//                                         dom::attributes!{
//                                             "id" => "canvas",
//                                             "height" => "730",
//                                             "width" => "980",
//                                         },
//                                         vec![],
//                                     ),
//                                     generate_toolbar(),
//                                 ],
//                             ),
//                             dom::panel::create(
//                                 "panel__simulation",
//                                 "view__simulations",
//                                 vec![
//                                     ("Simulation".to_string(), vec![dom::create_element(
//                                         "canvas",
//                                         dom::attributes!{
//                                             "id" => "simulation__canvas",
//                                             "height" => "250",
//                                             "width" => "325",
//                                         },
//                                         vec![],
//                                     )]),
//                                 ]
//                             )
//                         ],
//                     ),
//                     righ_menu::generate(),
//                 ]
//             )
//         ]
//     );
// }
// 
// mod righ_menu {
//     use crate::dom;
//     use crate::unit;
// 
//     pub fn generate() -> web_sys::Element {
//         dom::create_element(
//             "div",
//             dom::attributes!{ "class" => "sidebar__container" },
//             vec![
//                 dom::create_element(
//                     "div",
//                     dom::attributes!{ "class" => "tabs__container" },
//                     vec![
//                         properties_tab(),
//                         simulation_tab(),
//                     ]
//                 )
//             ]
//         )
//     }
// 
//     fn properties_tab() -> web_sys::Element {
//         dom::create_element(
//             "div",
//             dom::attributes!{ "class" => "tab__container" },
//             vec![
//                 dom::create_element(
//                     "input",
//                     dom::attributes!{
//                         "type" => "radio",
//                         "name" => "tab__right-menu",
//                         "id" => "tab__right-menu-properties",
//                         "checked" => "",
//                     },
//                     vec![],
//                 ),
//                 dom::create_element(
//                     "label",
//                     dom::attributes!{
//                         "class" => "form__label-bold",
//                         "for" => "tab__right-menu-properties",
//                         "inner_html" => "Propriétés",
//                     },
//                     vec![],
//                 ),
//                 dom::create_element(
//                     "div",
//                     dom::attributes!{ "class" => "tab__content" },
//                     vec![
//                         dom::create_element(
//                             "div",
//                             dom::attributes!{ "class" => "form__container" },
//                             vec![
//                                 dom::create_element(
//                                     "div",
//                                     dom::attributes!{
//                                         "id" => "property__container"
//                                     },
//                                     vec![],
//                                 ),
//                                 dom::create_element(
//                                     "button",
//                                     dom::attributes!{
//                                         "class" => "form__button",
//                                         "id" => "property__save-button",
//                                         "inner_html" => "Sauvegarder",
//                                     },
//                                     vec![],
//                                 )
//                             ]
//                         )
//                     ],
//                 )
//             ]
//         )
//     }
// 
//     fn simulation_tab() -> web_sys::Element {
//         dom::create_element(
//             "div",
//             dom::attributes!{ "class" => "tab__container" },
//             vec![
//                 dom::create_element(
//                     "input",
//                     dom::attributes!{
//                         "type" => "radio",
//                         "name" => "tab__right-menu",
//                         "id" => "tab__right-menu-simulation",
//                     },
//                     vec![],
//                 ),
//                 dom::create_element(
//                     "label",
//                     dom::attributes!{
//                         "class" => "form__label-bold",
//                         "for" => "tab__right-menu-simulation",
//                         "inner_html" => "Simulations",
//                     },
//                     vec![],
//                 ),
//                 dom::create_element(
//                     "div",
//                     dom::attributes!{ "class" => "tab__content" },
//                     vec![
//                         dom::create_element(
//                             "div",
//                             dom::attributes!{ "class" => "form__container" },
//                             vec![
//                                 dom::form::label::create("Node (simulation)"),
//                                 dom::form::select::create("tran__node", Vec::new(), 0),
//                                 dom::form::label::create("Step"),
//                                 dom::form::group(vec![
//                                     dom::form::text_input::create("tran__step", "1"),
//                                     dom::form::select::create_unit("tran__step-prefix", &unit::Unit::Second, 3),
//                                 ]),
//                                 dom::form::label::create("Stop"),
//                                 dom::form::group(vec![
//                                     dom::form::text_input::create("tran__stop", "100"),
//                                     dom::form::select::create_unit("tran__stop-prefix", &unit::Unit::Second, 3),
//                                 ]),
//                             ]
//                         )
//                     ],
//                 )
//             ]
//         )
// 
//     }
// }

