use crate::dom;

pub fn group (elements: Vec<web_sys::Element>) -> web_sys::Element {
    dom::create_element(
        "div",
        dom::attributes!{ "class" => "form__group" },
        elements,
    )
}

pub mod button {
    use crate::dom;

    pub fn create(title: &str) -> web_sys::Element {
        let button = dom::create_element("button", dom::attributes!{ "class" => "form__button dark" }, vec![]);
        button.set_inner_html(title);
        button
    }
}

pub mod radio {
    use crate::dom;

    pub fn create(name: &str, value: &str, icon: &str) -> web_sys::Element {
        dom::create_element(
            "label",
            dom::attributes!{ "class" => "form__radio" },
            vec![
                dom::create_element(
                    "input",
                    dom::attributes!{
                        "type" => "radio",
                        "name" => name,
                        "value" => value,
                    },
                    vec![],
                ),
                dom::create_element("span", dom::attributes!{}, vec![
                    dom::inner_html(icon)
                ]),
            ]
        )
    }

    pub fn create_from_vec(name: &str, values: Vec<&str>, icons: Vec<&str>) -> Vec<web_sys::Element> {
        values.iter().enumerate().map(|(idx, value)| {
            create(name, value, icons[idx])
        }).collect::<Vec<web_sys::Element>>()
    }
}

pub mod label {
    use crate::dom;

    pub fn create(title: &str) -> web_sys::Element {
        let label = dom::create_element("label", dom::attributes!{ "class" => "form__label" }, vec![]);
        label.set_inner_html(title);
        label
    }
}

pub mod text_input {
    use crate::dom;
    use wasm_bindgen::JsCast;

    pub fn create(name: &str, value: &str) -> web_sys::Element {
        dom::create_element(
            "input",
            dom::attributes!{ 
                "type" => "text",
                "name" => name,
                "class" => "form__text-input",
                "value" => value,
            },
            vec![],
        )
    }

    pub fn value_as_string(selector: &str) -> Result<String, ()> {
        if let Ok(Some(element)) = dom::document().query_selector(selector) {
            let element: web_sys::HtmlInputElement = element
                .dyn_into::<web_sys::HtmlInputElement>()
                .map_err(|_| ())
                .unwrap();
            Ok(element.value())
        } else {
            Err(())
        }
    }
}

pub mod checkbox {
    use crate::dom;
    use wasm_bindgen::JsCast;

    pub fn create(name: &str, is_checked: bool, icon: &str) -> web_sys::Element {
        let checkbox = dom::create_element(
            "input",
            dom::attributes!{
                "type" => "checkbox",
                "name" => name,
            },
            vec![],
        );
        if is_checked {
            checkbox.set_attribute("checked", "").unwrap();
        }
        dom::create_element(
            "label",
            dom::attributes!{ "class" => "form__toggle-input" },
            vec![
                checkbox,
                dom::create_element("span", dom::attributes!{
                    "inner_html" => icon
                }, vec![]),
            ]
        )
    }

    pub fn value_as_bool(selector: &str) -> Result<bool, ()> {
        if let Ok(Some(element)) = dom::document().query_selector(selector) {
            let element: web_sys::HtmlInputElement = element
                .dyn_into::<web_sys::HtmlInputElement>()
                .map_err(|_| ())
                .unwrap();
            Ok(element.checked())
        } else {
            Err(())
        }
    }
}

pub mod select {
    use crate::dom;
    use wasm_bindgen::JsCast;
    use crate::unit;

    pub fn create(name: &str, options: Vec<(String, String)>, selected: usize) -> web_sys::Element {
        let options_element = create_options(options, selected);
        dom::create_element(
            "div",
            dom::attributes!{ "class" => "form__select" },
            vec![
                dom::create_element(
                    "select",
                    dom::attributes! { "name" => name },
                    options_element,
                ),
            ]
        )
    }

    fn create_options(options: Vec<(String, String)>, selected: usize) -> Vec<web_sys::Element> {
        options
            .iter()
            .enumerate()
            .map(|(idx, (value, text))| {
                let option = dom::create_element(
                    "option",
                    dom::attributes!{ "value" => value.as_str(), "inner_html" => text },
                    vec![]
                );
                if idx == selected {
                    option.set_attribute("selected", "").unwrap();
                }
                option
            })
            .collect::<Vec<web_sys::Element>>()

    }

    pub fn create_unit(name: &str, unit: &unit::Unit, selected: usize) -> web_sys::Element {
        let options = unit::Prefix::as_array()
            .iter()
            .enumerate()
            .map(|(idx, prefix)| {
                (idx.to_string(), format!("{}{}", prefix.to_string(), unit.to_string()))
            }).collect::<Vec<(String, String)>>();
        create(name, options, selected)
    }

    pub fn update_options(name: &str, options: Vec<(String, String)>) {
        let select = dom::select(&format!("[name=\"{}\"]", name));
        select.set_inner_html("");
        dom::append_children(
            &select,
            create_options(options, 0)
                .iter()
                .collect::<Vec<&web_sys::Element>>()
        );
    }

    pub fn value_as_string(selector: &str) -> Result<String, ()> {
        if let Ok(Some(element)) = dom::document().query_selector(selector) {
            let element: web_sys::HtmlSelectElement = element
                .dyn_into::<web_sys::HtmlSelectElement>()
                .map_err(|_| ())
                .unwrap();
            Ok(element.value())
        } else {
            Err(())
        }
    }

    pub fn value_as_usize(selector: &str) -> Result<usize, ()> {
        if let Ok(value) = value_as_string(selector) {
            if let Ok(value_usize) = value.parse::<usize>() {
                return Ok(value_usize)
            }
        }
        Err(())
    }
}
