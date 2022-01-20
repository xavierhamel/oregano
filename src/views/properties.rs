use crate::dom;
use crate::schema::parts::part;

pub fn update(part: &part::Part) {
    let container = dom::select("#menu__property-list");
    container.set_inner_html("");

    part.properties
        .clone()
        .into_iter()
        .for_each(|(key, property)| {
            dom::append_children(
                &container,
                vec![
                    &dom::create_element(
                        "div",
                        dom::attributes! { "class" => "form__tooltip-group" },
                        vec![
                            dom::form::label::new(&key, dom::attributes! {}),
                            dom::form::tooltip::create(""),
                        ],
                    ),
                    &property.into_input(&key),
                ],
            );
        });
}

pub fn empty() {
    dom::select("#menu__property-list")
        .set_inner_html("<label class=\"form__label-bold\">Aucun composant sélectionné</label>");
}
