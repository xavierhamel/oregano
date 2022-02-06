use crate::dom;
use crate::error;

/// Will parse the input string and return the value as the type needed. Use this function for
/// parsing values of html inputs because of the corresponding error retuned if it fails.
fn parse<T: std::str::FromStr>(input: &str) -> Result<T, error::Error> {
    if let Ok(value) = input.parse::<T>() {
        Ok(value)
    } else {
        Err(Box::new(error::Dom::InputType(
            std::any::type_name::<T>().to_string(),
        )))
    }
}

pub fn group(elements: Vec<web_sys::Element>) -> web_sys::Element {
    dom::create_element(
        "div",
        dom::attributes! { "class" => "form__group" },
        elements,
    )
}

pub mod button {
    use crate::dom;

    pub fn create(title: &str) -> web_sys::Element {
        let button = dom::create_element(
            "button",
            dom::attributes! { "class" => "form__button dark" },
            vec![],
        );
        button.set_inner_html(title);
        button
    }
}

pub mod radio {
    use crate::{dom, error};

    pub fn create(name: &str, value: &str, icon: &str, is_checked: bool) -> web_sys::Element {
        let mut attributes = dom::attributes! {
            "type" => "radio",
            "name" => name,
            "value" => value,
        };
        if is_checked {
            attributes.insert("checked", "");
        }
        dom::create_element(
            "label",
            dom::attributes! { "class" => "form__radio" },
            vec![
                dom::create_element("input", attributes, vec![]),
                dom::create_element("span", dom::attributes! {}, vec![dom::inner_html(icon)]),
            ],
        )
    }

    pub fn create_from_vec(
        name: &str,
        values: Vec<&str>,
        icons: Vec<&str>,
    ) -> Vec<web_sys::Element> {
        values
            .iter()
            .enumerate()
            .map(|(idx, value)| create(name, value, icons[idx], idx == 0))
            .collect::<Vec<web_sys::Element>>()
    }

    pub fn value<T: std::str::FromStr>(elements: Vec<web_sys::Element>) -> Result<T, error::Error> {
        for element in elements.into_iter() {
            let input = dom::convert::<web_sys::HtmlInputElement>(element)?;
            if input.checked() {
                return super::parse::<T>(&input.value());
            }
        }
        super::parse::<T>("")
    }

    pub fn set_checked<T: std::fmt::Display>(selector: &str, value: T) -> Result<(), error::Error> {
        let string_value = value.to_string();
        for element in dom::select_all(selector).into_iter() {
            let input = dom::convert::<web_sys::HtmlInputElement>(element)?;
            if input.value() == string_value {
                input.set_checked(true);
            } else {
                input.set_checked(false);
            }
        }
        Ok(())
    }
}

pub mod label {
    use crate::dom;
    use std::collections::HashMap;

    #[deprecated(note = "`new` should be used instead of `create`")]
    pub fn create(title: &str) -> web_sys::Element {
        dom::create_element(
            "label",
            dom::attributes! {
                "class" => "form__label",
                "inner_html" => title,
            },
            vec![],
        )
    }

    pub fn new<'a>(title: &'a str, mut attributes: HashMap<&str, &'a str>) -> web_sys::Element {
        if !attributes.contains_key("class") {
            attributes.insert("class", "form__label");
        }
        attributes.insert("inner_html", title);
        dom::create_element("label", attributes, vec![])
    }
}

pub mod text_input {
    use crate::dom;
    use crate::error;
    use std::collections::HashMap;

    #[deprecated(note = "`new` should be used instead of `create`")]
    pub fn create(name: &str, value: &str) -> web_sys::Element {
        dom::create_element(
            "input",
            dom::attributes! {
                "type" => "text",
                "name" => name,
                "class" => "form__text-input",
                "value" => value,
            },
            vec![],
        )
    }

    pub fn new<'a>(mut attributes: HashMap<&str, &'a str>) -> web_sys::Element {
        attributes.insert("class", "form__text-input");
        attributes.insert("type", "text");
        dom::create_element("input", attributes, vec![])
    }

    pub fn hidden<'a>(mut attributes: HashMap<&str, &'a str>) -> web_sys::Element {
        attributes.insert("type", "hidden");
        dom::create_element("input", attributes, vec![])
    }

    pub fn value<T: std::str::FromStr>(element: web_sys::Element) -> Result<T, error::Error> {
        let input = dom::convert::<web_sys::HtmlInputElement>(element)?;
        super::parse::<T>(&input.value())
    }
}

pub mod checkbox {
    use crate::dom;
    use crate::error;
    use std::collections::HashMap;

    pub fn create(name: &str, is_checked: bool, icon: &str) -> web_sys::Element {
        let checkbox = dom::create_element(
            "input",
            dom::attributes! {
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
            dom::attributes! { "class" => "form__toggle-input" },
            vec![
                checkbox,
                dom::create_element(
                    "span",
                    dom::attributes! {
                        "inner_html" => icon
                    },
                    vec![],
                ),
            ],
        )
    }

    pub fn new<'a>(
        icon: &'a str,
        mut attributes: HashMap<&str, &'a str>,
        is_checked: bool,
    ) -> web_sys::Element {
        attributes.insert("type", "checkbox");
        if is_checked {
            attributes.insert("checked", "");
        }
        let checkbox = dom::create_element("input", attributes, vec![]);
        dom::create_element(
            "label",
            dom::attributes! { "class" => "form__toggle-input" },
            vec![
                checkbox,
                dom::create_element(
                    "span",
                    dom::attributes! {
                        "inner_html" => icon
                    },
                    vec![],
                ),
            ],
        )
    }

    /// Dual icon is useful when you use an icon when the box is checked and an other one the box
    /// is unchecked.
    pub fn dual_icon(checked: &str, unchecked: &str) -> String {
        format!(
            "<div class=\"checked\">{}</div><div class=\"unchecked\">{}</div>",
            dom::icon(checked),
            dom::icon(unchecked)
        )
    }

    pub fn value(element: web_sys::Element) -> Result<bool, error::Error> {
        let input = dom::convert::<web_sys::HtmlInputElement>(element)?;
        Ok(input.checked())
    }
}

pub mod select {
    use crate::dom;
    use crate::error;
    use crate::unit;

    pub fn create(name: &str, options: Vec<(String, String)>, selected: usize) -> web_sys::Element {
        let options_element = create_options(options, selected);
        dom::create_element(
            "div",
            dom::attributes! { "class" => "form__select" },
            vec![dom::create_element(
                "select",
                dom::attributes! { "name" => name },
                options_element,
            )],
        )
    }

    fn create_options(options: Vec<(String, String)>, selected: usize) -> Vec<web_sys::Element> {
        options
            .iter()
            .enumerate()
            .map(|(idx, (value, text))| {
                let option = dom::create_element(
                    "option",
                    dom::attributes! { "value" => value.as_str(), "inner_html" => text },
                    vec![],
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
                (
                    idx.to_string(),
                    format!("{}{}", prefix.to_string(), unit.to_string()),
                )
            })
            .collect::<Vec<(String, String)>>();
        create(name, options, selected)
    }

    pub fn update_options(name: &str, options: Vec<(String, String)>) {
        let select = dom::select(&format!("[name=\"{}\"]", name));
        select.set_inner_html("");
        dom::append_children(
            &select,
            create_options(options, 0)
                .iter()
                .collect::<Vec<&web_sys::Element>>(),
        );
    }

    pub fn value<T: std::str::FromStr>(element: web_sys::Element) -> Result<T, error::Error> {
        let input = dom::convert::<web_sys::HtmlSelectElement>(element)?;
        super::parse::<T>(&input.value())
    }
}

pub mod tooltip {
    use crate::dom;

    pub fn create(description: &Option<String>) -> web_sys::Element {
        if let Some(text) = description {
            dom::create_element(
                "div",
                dom::attributes! {
                    "class" => "form__tooltip",
                    "inner_html" => "?",
                },
                vec![dom::create_element(
                    "div",
                    dom::attributes! {
                        "class" => "form__tooltip-description",
                        "inner_html" => text,
                    },
                    vec![],
                )],
            )
        } else {
            dom::create_element("div", dom::attributes! {}, vec![])
        }
    }
}
