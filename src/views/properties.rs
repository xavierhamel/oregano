use crate::schema::{parts::part, properties};
use crate::{dom, PROPS_DATA};

pub fn update(part: &part::Part) {
    let properties_container = dom::select("#menu__properties");
    let mut is_model_present = false;
    dom::select("#menu__model-properties").set_inner_html(
        "<h2>Propriétés du modèle</h2><div class=\"panel__model-properties\"></div>",
    );
    let model_container = dom::select("#menu__model-properties > div");
    model_container.set_inner_html("");
    properties_container.set_inner_html("");

    part.properties.clone().into_iter().for_each(|property| {
        if property.1.is_model {
            is_model_present = true;
            add_model_property(&model_container, property);
        } else {
            add_property(&properties_container, property);
        }
    });
    if !is_model_present {
        dom::select("#menu__model-properties").set_inner_html("");
    }
}

fn add_model_property(container: &web_sys::Element, prop: (String, properties::Property)) {
    let (key, property) = prop;
    let label = if let Some(data) = PROPS_DATA.get(key.clone()) {
        let tooltip = dom::create_element(
            "div",
            dom::attributes! {
                "class" => "form__label",
                "inner_html" => &data.title,
            },
            vec![dom::create_element(
                "div",
                dom::attributes! {
                    "class" => "form__tooltip-description",
                    "inner_html" => &data.title,
                },
                vec![],
            )],
        );
        vec![
            //dom::form::label::new(&data.title, dom::attributes! {}),
            tooltip,
            property.into_input(&key),
        ]
    } else {
        vec![
            dom::form::label::new(&key, dom::attributes! {}),
            property.into_input(&key),
        ]
    };
    dom::append_children(
        &container,
        vec![&dom::create_element(
            "div",
            dom::attributes! { "class" => "form__compact-group" },
            label,
        )],
    );
}

fn add_property(container: &web_sys::Element, prop: (String, properties::Property)) {
    let (key, property) = prop;
    let label = if let Some(data) = PROPS_DATA.get(key.clone()) {
        vec![
            dom::form::label::new(&data.title, dom::attributes! {}),
            dom::form::tooltip::create(&data.description),
        ]
    } else {
        vec![dom::form::label::new(&key, dom::attributes! {})]
    };
    dom::append_children(
        &container,
        vec![
            &dom::create_element(
                "div",
                dom::attributes! { "class" => "form__tooltip-group" },
                label,
            ),
            &property.into_input(&key),
        ],
    );
}

pub fn empty() {
    dom::select("#menu__properties")
        .set_inner_html("<label class=\"form__label-bold\">Aucun composant sélectionné</label>");
    dom::select("#menu__model-properties").set_inner_html("");
}
