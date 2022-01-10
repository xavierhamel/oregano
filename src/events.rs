use crate::dom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use crate::editor;
use crate::intrinsics;
use crate::plot;
use crate::log;

/// This will add all the event necessary for the editor to work. There is probably a better way to
/// do this but I did not took the time to find it yet.
pub fn add_events(editor: Rc<RefCell<editor::Editor>>, canvas_id:&str) {
    let canvas = dom::canvas::as_canvas(dom::select(canvas_id));
    {
        //let canvas = canvas.clone();
        let canvas = editor::Editor::canvas(canvas_id);
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            editor.borrow_mut().scene.update_mouse(
                event.offset_x() as f64,
                event.offset_y() as f64
            );
            editor.borrow_mut().update();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(
            "mousemove",
            closure.as_ref().unchecked_ref()
        ).unwrap();
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
        canvas.add_event_listener_with_callback(
            "mouseup",
            closure.as_ref().unchecked_ref()
        ).unwrap();
        closure.forget();
    }
    {
        //let canvas = canvas.clone();
        let canvas = editor::Editor::canvas(canvas_id);
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            editor.borrow_mut().scene.mouse.mousedown(
                event.offset_x() as f64,
                event.offset_y() as f64
            );
            editor.borrow_mut().update();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(
            "mousedown",
            closure.as_ref().unchecked_ref()
        ).unwrap();
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
        canvas.add_event_listener_with_callback(
            "wheel",
            closure.as_ref().unchecked_ref()
        ).unwrap();
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
        dom::document().add_event_listener_with_callback(
            "keydown",
            closure.as_ref().unchecked_ref()
        ).unwrap();
        closure.forget();
    }
    {
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {
            let plot = plot::Plot::new(
                plot::parse_spice_output(event.detail().as_string().unwrap()),
                "time".to_string(),
                "V".to_string(),
                intrinsics::Color("#0000FF"),
                "#simulation__canvas".to_string(),
            );
            plot.draw();
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().add_event_listener_with_callback(
            "ngspice_response",
            closure.as_ref().unchecked_ref()
        ).unwrap();
        closure.forget();
    }

    // Events for the toolbar
    {
        let editor = editor.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {
            editor.borrow_mut().toolbar_updated();
        }) as Box<dyn FnMut(_)>);
        for name in vec!["mouse", "components", "properties", "simulation"].iter() {
            dom::select(&format!("[name=\"toolbar__{}\"]", name)).add_event_listener_with_callback(
                "change",
                closure.as_ref().unchecked_ref()
            ).unwrap();
            web_sys::window().unwrap().add_event_listener_with_callback(
                "resize",
                closure.as_ref().unchecked_ref()
            ).unwrap();
        }
        closure.forget();
    }
    {
        let closure = Closure::wrap(Box::new(move |_: web_sys::MutationRecord| {
            let editor = editor.clone();
            let change_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
                editor.borrow_mut().scene.entities.update_selected_entity_properties();
                editor.borrow_mut().update();
            }) as Box<dyn FnMut(_)>);
            for element in dom::select_all("[name^=\"property__\"]").iter() {
                element.add_event_listener_with_callback(
                    "change",
                    change_closure.as_ref().unchecked_ref()
                ).unwrap();
            }
            change_closure.forget();
        }) as Box<dyn FnMut(_)>);
        let mut config = web_sys::MutationObserverInit::new();
        config.child_list(true);
        web_sys::MutationObserver::new(
            closure.as_ref().unchecked_ref()
        ).unwrap().observe_with_options(&dom::select("#menu__property-list"), &config);
        closure.forget();

    }
}
