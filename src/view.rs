use crate::dom;
use crate::unit;

pub fn generate_toolbar() {
    let tools = dom::create_element(
        "div",
        dom::attributes! { "class" => "flex-row" },
        dom::form::radio::create_from_vec(
            "toolbar__mouse",
            vec!["mouse", "wire"],
            vec![
                "<i class=\"fas fa-mouse-pointer\"></i>",
                "<i class=\"fas fa-draw-polygon\"></i>",
            ],
        ),
    );
    dom::append_children(
        &dom::select("#menu__toolbar-container"),
        vec![
            &dom::create_element(
                "button",
                dom::attributes! {
                    "class" => "form__button-square",
                    "inner_html" => "<i class=\"fas fa-save\"></i>",
                },
                vec![],
            ),
            &dom::create_element(
                "button",
                dom::attributes! {
                    "class" => "form__button-square",
                    "inner_html" => "<i class=\"fas fa-file-download\"></i>"
                },
                vec![],
            ),
            &dom::create_element(
                "div",
                dom::attributes! { "class" => "toolbar__separator" },
                vec![],
            ),
            &tools,
            &dom::create_element(
                "div",
                dom::attributes! { "class" => "toolbar__separator" },
                vec![],
            ),
            &dom::form::checkbox::create(
                "toolbar__components",
                true,
                "<i class=\"fas fa-bolt\"></i>",
            ),
            &dom::form::checkbox::create(
                "toolbar__properties",
                true,
                "<i class=\"fas fa-align-justify\"></i>",
            ),
            &dom::form::checkbox::create(
                "toolbar__simulation",
                true,
                "<i class=\"fas fa-cog\"></i>",
            ),
        ],
    );

    dom::append_children(
        &dom::select("#menu__toolbar-fullscreen-result-container"),
        vec![&dom::form::checkbox::new(
            &dom::form::checkbox::dual_icon("expand-alt", "compress-alt"),
            dom::attributes! { "name" => "toolbar__editor", "class" => "form__toggle-input-dual-icon" },
            true,
        )],
    );
}

pub fn generate_simulation() {
    dom::append_children(
        &dom::select("#menu__simulations-type"),
        vec![
            &dom::form::label::create("Type d'analyse"),
            &dom::form::select::create(
                "sim__type",
                vec![
                    ("tran".to_string(), "Transitoire".to_string()),
                    ("freq".to_string(), "Fréquence".to_string()),
                    //("op".to_string(), "Point d'opération".to_string()),
                ],
                0,
            ),
        ],
    );

    // TRAN SIMULATION
    dom::append_children(
        &dom::select("#menu__simulations-type-tran"),
        vec![
            &dom::form::label::create("Step"),
            &dom::form::group(vec![
                dom::form::text_input::new(dom::attributes! {
                    "name" => "tran__step",
                    "data-property-type" => "unit",
                    "value" => "1"
                }),
                dom::form::select::create_unit("tran__step-unit-prefix", &unit::Unit::Second, 3),
                dom::form::text_input::hidden(
                    dom::attributes! { "name" => "tran__step-unit", "value" => "s" },
                ),
            ]),
            &dom::form::label::create("Stop"),
            &dom::form::group(vec![
                dom::form::text_input::new(dom::attributes! {
                    "name" => "tran__stop",
                    "data-property-type" => "unit",
                    "value" => "100"
                }),
                dom::form::select::create_unit("tran__stop-unit-prefix", &unit::Unit::Second, 3),
                dom::form::text_input::hidden(
                    dom::attributes! { "name" => "tran__stop-unit", "value" => "s" },
                ),
            ]),
        ],
    );
    // FREQ SIMULATION
    dom::append_children(
        &dom::select("#menu__simulations-type-freq"),
        vec![
            &dom::form::label::create("Variation type"),
            &dom::form::select::create(
                "sim__freq-variation-type",
                vec![
                    ("dec".to_string(), "Decade".to_string()),
                    ("oct".to_string(), "Octave".to_string()),
                    ("lin".to_string(), "Linear".to_string()),
                ],
                0,
            ),
            &dom::form::label::create("Number of point (between variation)"),
            &dom::form::text_input::create("sim__freq-np", "10"),
            &dom::form::label::create("Starting Frequency"),
            &dom::form::group(vec![
                dom::form::text_input::new(dom::attributes! {
                    "name" => "sim__freq-fstart",
                    "data-property-type" => "unit",
                    "value" => "1"
                }),
                dom::form::select::create_unit(
                    "sim__freq-fstart-unit-prefix",
                    &unit::Unit::Hertz,
                    3,
                ),
                dom::form::text_input::hidden(
                    dom::attributes! { "name" => "sim__freq-fstart-unit", "value" => "Hz" },
                ),
            ]),
            &dom::form::label::create("Final Frequency"),
            &dom::form::group(vec![
                dom::form::text_input::new(dom::attributes! {
                    "name" => "sim__freq-fstop",
                    "data-property-type" => "unit",
                    "value" => "10"
                }),
                dom::form::select::create_unit(
                    "sim__freq-fstop-unit-prefix",
                    &unit::Unit::Hertz,
                    2,
                ),
                dom::form::text_input::hidden(
                    dom::attributes! { "name" => "sim__freq-fstop-unit", "value" => "Hz" },
                ),
            ]),
            &dom::form::label::create("Data Type"),
            &dom::form::select::create(
                "sim__freq-data-type",
                vec![
                    ("db".to_string(), "Decibel (dB)".to_string()),
                    ("p".to_string(), "Phase".to_string()),
                    ("r".to_string(), "Real part".to_string()),
                    ("i".to_string(), "Imaginary part".to_string()),
                    ("m".to_string(), "Magnitude".to_string()),
                ],
                0,
            ),
        ],
    );
}
