use crate::{dom, editor, unit};
use crate::editor::{entity::Entity, component::components, entity};
use crate::intrinsics::*;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::{prelude::*, JsCast};

/// Functions in this file are used to generate different dialogs window related to components.

/// Generate the list of components at the left of the editor.
pub fn load_components_dialog(editor: Rc<RefCell<editor::Editor>>) {
    const BUTTON_WIDTH: f64 = 65.0;
    const BUTTON_HEIGHT: f64 = 45.0;
    const BUTTON_RATIO: f64 = 0.75;

    let container = dom::select("#menu__component-list");
    let sections = components::Components::as_sections();

    for (title, components) in sections.iter() {
        let width_string = BUTTON_WIDTH.to_string();
        let height_string = BUTTON_HEIGHT.to_string();
        let buttons = components
            .iter()
            .map(|comp| {
                let mut component = comp.generate(0);
                let canvas = dom::canvas::create(
                    dom::attributes!{
                        "width" => &width_string[..],
                        "height" => &height_string[..],
                    }
                );
                let context = dom::canvas::context(&dom::canvas::as_canvas(canvas.clone()));
                let new_size = Size::new(
                    component.shape().size().w * BUTTON_RATIO,
                    component.shape().size().h * BUTTON_RATIO
                );
                context.translate(
                    (BUTTON_WIDTH - new_size.w) / 2.0,
                    (BUTTON_HEIGHT - new_size.h) / 2.0
                ).unwrap();
                context.scale(BUTTON_RATIO, BUTTON_RATIO).unwrap();
                component.properties = std::collections::hash_map::HashMap::new();
                component.draw(&context);
                let button = dom::create_element(
                    "div",
                    dom::attributes!{ "class" => "dialog-component-button" },
                    vec![
                        canvas,
                        dom::form::label::create(component.short_name),
                    ]
                );
                {
                    let editor = editor.clone();
                    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                        editor.borrow_mut().scene.entities.add_floating_component(&component.typ);
                        editor.borrow_mut().update();
                    }) as Box<dyn FnMut(_)>);
                    button.add_event_listener_with_callback(
                        "click",
                        closure.as_ref().unchecked_ref()
                    ).unwrap();
                    closure.forget();
                }
                button
            }).collect::<Vec<web_sys::Element>>();
        dom::append_children(
            &container,
            vec![
                &dom::form::label::create(title),
                &dom::create_element(
                    "div",
                    dom::attributes!{
                        "class" => "flex-row dialog-components",
                    },
                    buttons
                )
            ]
        );
    }
}

pub fn load_properties_dialog(component: &Box<dyn Entity>) {
    let container = dom::select("#menu__property-list");
    container.set_inner_html("");
    for (key, property) in component.properties().iter() {
        let group = match property {
            entity::Property::Text(name, is_visible) => {
                dom::form::group(
                    vec![
                        dom::form::text_input::create(&format!("property__{}", key), &name.to_string()),
                        dom::form::checkbox::create(
                            &format!("property__{}-is-visible", key),
                            *is_visible,
                            "<i class=\"fas fa-eye\"></i>"
                        ),
                    ]
                )
            },
            entity::Property::Num(number, is_visible) => {
                dom::form::group(
                    vec![
                        dom::form::text_input::create(&format!("property__{}", key), &number.to_string()),
                        dom::form::checkbox::create(
                            &format!("property__{}-is-visible", key),
                            *is_visible,
                            "<i class=\"fas fa-eye\"></i>"
                        ),
                    ]
                )
            },
            entity::Property::Unit(value, prefix, unit, is_visible) => {
                let unit_string = unit.to_string();
                let hidden_name = format!("property__{}-unit", key);
                dom::form::group(
                    vec![
                        dom::create_element(
                            "input",
                            dom::attributes!{ 
                                "type" => "hidden", 
                                "value" => &unit_string,
                                "name" => &hidden_name,
                            },
                            vec![]
                        ),
                        dom::form::text_input::create(&format!("property__{}", key), &value.to_string()),
                        dom::form::select::create_unit(
                            &format!("property__{}-unit-prefix", key),
                            unit,
                            unit::Prefix::as_array().iter().position(|p| p == prefix).unwrap()
                        ),
                        dom::form::checkbox::create(
                            &format!("property__{}-is-visible", key),
                            *is_visible,
                            "<i class=\"fas fa-eye\"></i>"
                        ),
                    ]
                )
            },
            // Internal properties cannot be changed by the user.
            entity::Property::InternalStr(_) | entity::Property::InternalF64(_) => { continue; }
        };
        dom::append_children(
            &container,
            vec![
                &dom::form::label::create(&key),
                &group,
            ]
        );
    }
}

pub fn empty_properties_dialog() {
    dom::select("#menu__property-list").set_inner_html("<label class=\"form__label-bold\">Aucun composant sélectionné</label>");
}
