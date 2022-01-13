use crate::dom;
use crate::editor;
use crate::intrinsics;
use crate::intrinsics::*;
use crate::plot;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// This will add all the event necessary for the editor to work. There is probably a better way to
/// do this but I did not took the time to find it yet.
pub fn add_events(editor: Rc<RefCell<editor::Editor>>, canvas_id: &str) {
    let canvas = dom::canvas::as_canvas(dom::select(canvas_id));
    {
        //let canvas = canvas.clone();
        let canvas = editor::Editor::canvas(canvas_id);
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            editor
                .borrow_mut()
                .scene
                .update_mouse(event.offset_x() as f64, event.offset_y() as f64);
            editor.borrow_mut().update();
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        //let canvas = canvas.clone();
        let canvas = editor::Editor::canvas(canvas_id);
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            editor.borrow_mut().scene.mouse.mouseup();
            editor.borrow_mut().update();
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        //let canvas = canvas.clone();
        let canvas = editor::Editor::canvas(canvas_id);
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            editor
                .borrow_mut()
                .scene
                .mouse
                .mousedown(event.offset_x() as f64, event.offset_y() as f64);
            editor.borrow_mut().update();
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    // {
    //     let save_button = dom::select("#property__save-button");
    //     let editor = editor.clone();
    //     let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    //         editor.borrow_mut().scene.entities.update_selected_entity_properties();
    //         editor.borrow_mut().update();
    //     }) as Box<dyn FnMut(_)>);
    //     save_button.add_event_listener_with_callback(
    //         "click",
    //         closure.as_ref().unchecked_ref()
    //     ).unwrap();
    //     closure.forget();
    // }
    {
        //let canvas = canvas.clone();
        let canvas = editor::Editor::canvas(canvas_id);
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            editor.borrow_mut().scene.update_scale(event.delta_y());
            editor.borrow_mut().update();
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            editor.borrow_mut().scene.keystroke(event.key());
            editor.borrow_mut().update();
            if event.key() == "s".to_string() {
                editor.borrow().to_spice();
            }
        }) as Box<dyn FnMut(_)>);
        dom::document()
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {
            let (x_label, y_labels, series) =
                plot::parser::parse_spice_output(&event.detail().as_string().unwrap());
            editor
                .borrow_mut()
                .plots
                .update_data(series, x_label, y_labels);
            //let plot = plot::Plot::new(series, x_label, y_labels);
            //plot.draw();
            //editor.borrow_mut().set_plot(plot);
        }) as Box<dyn FnMut(_)>);
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("ngspice_response", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let canvas = dom::canvas::as_canvas(dom::select(plot::CANVAS_ID));
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            editor.borrow_mut().plots.mouse =
                Some(Point::new(event.offset_x() as f64, event.offset_y() as f64));
            editor.borrow_mut().plots.draw();
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let canvas = dom::canvas::as_canvas(dom::select(plot::CANVAS_ID));
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            editor.borrow_mut().plots.mouse = None;
            editor.borrow_mut().plots.draw();
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            for element in dom::select_all("[id^=\"menu__simulations-type-\"]") {
                element.set_attribute("class", "hide").unwrap();
            }
            dom::select(&format!(
                "#menu__simulations-type-{}",
                dom::form::select::value_as_string("[name=\"sim__type\"]").unwrap()
            ))
            .set_attribute("class", "")
            .unwrap();
        }) as Box<dyn FnMut(_)>);
        dom::select("[name=\"sim__type\"]")
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // Events for the toolbar
    {
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::CustomEvent| {
            editor.borrow_mut().toolbar_updated();
            editor.borrow_mut().plots.resize_canvas();
        }) as Box<dyn FnMut(_)>);
        for name in vec!["mouse", "components", "properties", "simulation"].iter() {
            dom::select(&format!("[name=\"toolbar__{}\"]", name))
                .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                .unwrap();
        }
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MutationRecord| {
            let editor = editor.clone();
            let change_closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                editor
                    .borrow_mut()
                    .scene
                    .entities
                    .update_selected_entity_properties();
                editor.borrow_mut().update();
            }) as Box<dyn FnMut(_)>);
            for element in dom::select_all("[name^=\"property__\"]").iter() {
                element
                    .add_event_listener_with_callback(
                        "change",
                        change_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
            change_closure.forget();
        }) as Box<dyn FnMut(_)>);
        let mut config = web_sys::MutationObserverInit::new();
        config.child_list(true);
        web_sys::MutationObserver::new(closure.as_ref().unchecked_ref())
            .unwrap()
            .observe_with_options(&dom::select("#menu__property-list"), &config)
            .unwrap();
        closure.forget();
    }
    {
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            dom::select("#error__container")
                .set_attribute("class", "hide")
                .unwrap();
        }) as Box<dyn FnMut(_)>);
        dom::select("#error__close")
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            editor.borrow_mut().plots.update_visible_series();
        }) as Box<dyn FnMut(_)>);
        dom::select("#sim__result-settings")
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            editor.borrow_mut().plots.select();
        }) as Box<dyn FnMut(_)>);
        dom::select("#sim__results-selector")
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            editor.borrow_mut().plots.add_plot();
        }) as Box<dyn FnMut(_)>);
        dom::select("#sim__results-add")
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}
