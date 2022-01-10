use crate::dom;

pub fn create(
    name: &str,
    class_container: &str,
    tabs: Vec<(String, Vec<web_sys::Element>)>,
) -> web_sys::Element {
    let tabs = tabs.iter().enumerate().map(|(idx, (title, children))| {
        let id = format!("{}-{}", name, idx);
        dom::create_element(
            "div",
            dom::attributes!{ "class" => "tab__container" },
            vec![
                dom::create_element(
                    "input",
                    dom::attributes!{
                        "type" => "radio",
                        "name" => name,
                        "id" => &id,
                        "checked" => "",
                    },
                    vec![],
                ),
                dom::create_element(
                    "label",
                    dom::attributes!{
                        "class" => "form__label-bold",
                        "for" => &id,
                        "inner_html" => &title,
                    },
                    vec![],
                ),
                dom::create_element(
                    "div",
                    dom::attributes!{ "class" => "tab__content" },
                    children.to_vec()
                )
            ]
        )
    }).collect::<Vec<web_sys::Element>>();
    dom::create_element(
        "div",
        dom::attributes!{ "class" => class_container },
        vec![
            dom::create_element(
                "div",
                dom::attributes!{ "class" => "tabs__container" },
                tabs
            )
        ]
    )
}
