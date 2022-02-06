use crate::intrinsics::*;
use crate::{dom, error, plot, schema, PARTS};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub enum Event {
    MouseMove,
    MouseDown,
    MouseUp,
    MouseLeave,
    Wheel,
    KeyDown,
    Click,
    Change,
    Resize,
    NgspiceResponse,
    NgspiceRequest,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::MouseMove => "mousemove",
            Self::MouseUp => "mouseup",
            Self::MouseDown => "mousedown",
            Self::MouseLeave => "mouseleave",
            Self::Wheel => "wheel",
            Self::KeyDown => "keydown",
            Self::Click => "click",
            Self::Change => "change",
            Self::Resize => "resize",
            Self::NgspiceRequest => "ngspice_request",
            Self::NgspiceResponse => "ngspice_response",
        };
        write!(f, "{}", out)
    }
}

impl std::str::FromStr for Event {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mousemove" => Ok(Self::MouseMove),
            "mouseup" => Ok(Self::MouseUp),
            "mousedown" => Ok(Self::MouseDown),
            "mouseleave" => Ok(Self::MouseLeave),
            "wheel" => Ok(Self::Wheel),
            "keydown" => Ok(Self::KeyDown),
            "click" => Ok(Self::Click),
            "change" => Ok(Self::Change),
            "resize" => Ok(Self::Resize),
            "ngspice_request" => Ok(Self::NgspiceRequest),
            "ngspice_response" => Ok(Self::NgspiceResponse),
            _ => Err(Box::new(error::Internal::Event)),
        }
    }
}

pub struct EventListener;

impl EventListener {
    pub fn add<T: wasm_bindgen::convert::FromWasmAbi + 'static>(
        element: &web_sys::EventTarget,
        event: &Event,
        closure: Box<dyn FnMut(T)>,
    ) {
        let c = Closure::wrap(closure);
        element
            .add_event_listener_with_callback(&event.to_string(), c.as_ref().unchecked_ref())
            .unwrap();
        c.forget();
    }

    pub fn add_multiple<T: wasm_bindgen::convert::FromWasmAbi + 'static>(
        element: &web_sys::EventTarget,
        events: Vec<Event>,
        closure: Box<dyn FnMut(web_sys::MouseEvent)>,
    ) {
        let c = Closure::wrap(closure);
        events.iter().for_each(|event| {
            element
                .add_event_listener_with_callback(&event.to_string(), c.as_ref().unchecked_ref())
                .unwrap();
        });
        c.forget();
    }

    pub fn add_mutation_observer(
        element: &web_sys::Element,
        closure: Box<dyn FnMut(web_sys::MutationRecord)>,
    ) {
        let c = Closure::wrap(closure);
        let mut config = web_sys::MutationObserverInit::new();
        config.child_list(true);
        web_sys::MutationObserver::new(c.as_ref().unchecked_ref())
            .unwrap()
            .observe_with_options(element, &config)
            .unwrap();
        c.forget();
    }
}

pub fn add_events_schema(schema: Rc<RefCell<schema::Schema>>, plots: Rc<RefCell<plot::Plots>>) {
    let canvas = dom::select(crate::SCHEMA_CANVAS_ID);
    //let canvas = dom::canvas::as_canvas(dom::select(crate::SCHEMA_CANVAS_ID));
    let s = schema.clone();
    EventListener::add_multiple::<web_sys::MouseEvent>(
        &canvas,
        vec![Event::MouseMove, Event::MouseUp, Event::MouseDown],
        Box::new(move |event: web_sys::MouseEvent| {
            if let Ok(event_type) = event.type_().parse::<Event>() {
                s.borrow_mut()
                    .mouse_dispatch(Point::from(event), event_type);
            }
        }),
    );

    let s = schema.clone();
    EventListener::add::<web_sys::WheelEvent>(
        &canvas,
        &Event::Wheel,
        Box::new(move |event: web_sys::WheelEvent| {
            s.borrow_mut().wheel_dispatch(event.delta_y());
        }),
    );

    let s = schema.clone();
    EventListener::add::<web_sys::KeyboardEvent>(
        &dom::document(),
        &Event::KeyDown,
        Box::new(move |event: web_sys::KeyboardEvent| {
            s.borrow_mut().keyboard_dispatch(&event.key());
        }),
    );

    dom::select_all("[data-part]")
        .into_iter()
        .for_each(|element| {
            let s = schema.clone();
            let e = element.clone();
            EventListener::add::<web_sys::MouseEvent>(
                &element,
                &Event::Click,
                Box::new(move |_event: web_sys::MouseEvent| {
                    let part = e.get_attribute("data-part").unwrap();
                    s.borrow_mut().add_part(PARTS.get(&part));
                }),
            )
        });
    [
        dom::select("#menu__properties"),
        dom::select("#menu__model-properties"),
    ]
    .iter()
    .for_each(|element| {
        let s = schema.clone();
        EventListener::add_mutation_observer(
            element,
            Box::new(move |_: web_sys::MutationRecord| {
                dom::select_all("[name^=\"property__\"]")
                    .into_iter()
                    .for_each(|element| {
                        let sc = s.clone();
                        EventListener::add::<web_sys::Event>(
                            &element,
                            &Event::Change,
                            Box::new(move |_event: web_sys::Event| {
                                sc.borrow_mut().properties_dispatch();
                            }),
                        );
                    });
            }),
        );
    });

    let s = schema.clone();
    let p = plots.clone();
    EventListener::add(
        &web_sys::window().unwrap(),
        &Event::Resize,
        Box::new(move |_event: web_sys::CustomEvent| {
            s.borrow_mut().resize_dispatch();
            p.borrow_mut().resize();
        }),
    );

    let p = plots.clone();
    EventListener::add(
        &dom::select(plot::CANVAS_ID),
        &Event::MouseMove,
        Box::new(move |event: web_sys::MouseEvent| {
            p.borrow_mut().mouse_updated(Some(Point::from(event)));
        }),
    );

    let p = plots.clone();
    EventListener::add(
        &dom::select(plot::CANVAS_ID),
        &Event::MouseLeave,
        Box::new(move |_: web_sys::MouseEvent| {
            p.borrow_mut().mouse_updated(None);
        }),
    );
    let p = plots.clone();
    let s = schema.clone();
    EventListener::add(
        &web_sys::window().unwrap(),
        &Event::NgspiceResponse,
        Box::new(move |event: web_sys::CustomEvent| {
            let (x_label, y_labels, series) = plot::parser::parse_spice_output(
                &s.borrow_mut().probes,
                &event.detail().as_string().unwrap(),
            );
            p.borrow_mut().update_data(series, x_label, y_labels);
        }),
    );

    let p = plots.clone();
    EventListener::add(
        &dom::select("#sim__result-settings"),
        &Event::Click,
        Box::new(move |_: web_sys::MouseEvent| p.borrow_mut().update_visible_series()),
    );
    let p = plots.clone();
    EventListener::add(
        &dom::select("#sim__results-selector"),
        &Event::Click,
        Box::new(move |_: web_sys::MouseEvent| p.borrow_mut().select()),
    );
    let p = plots.clone();
    EventListener::add(
        &dom::select("#sim__results-add"),
        &Event::Click,
        Box::new(move |_: web_sys::MouseEvent| p.borrow_mut().add_plot()),
    );

    dom::select_all("[name^=\"toolbar__\"]")
        .into_iter()
        .for_each(|element| {
            let s = schema.clone();
            let p = plots.clone();
            EventListener::add::<web_sys::Event>(
                &element,
                &Event::Change,
                Box::new(move |_: web_sys::Event| {
                    s.borrow_mut().toolbar_dispatch();
                    p.borrow_mut().resize();
                }),
            )
        });

    EventListener::add(
        &dom::select("#error__close"),
        &Event::Click,
        Box::new(move |_: web_sys::MouseEvent| {
            let _ = dom::select("#error__container").set_attribute("class", "hide");
        }),
    );

    let s = schema.clone();
    EventListener::add(
        &dom::select("[name=\"sim__type\"]"),
        &Event::Change,
        Box::new(move |_: web_sys::Event| {
            s.borrow_mut().simulation_selector_dispatch();
        }),
    );
}

// This will add all the event necessary for the editor to work. There is probably a better way to
// do this but I did not took the time to find it yet.
// pub fn add_events(editor: Rc<RefCell<editor::Editor>>, canvas_id: &str) {
//     let canvas = dom::canvas::as_canvas(dom::select(canvas_id));
//     {
//         //let canvas = canvas.clone();
//         let canvas = editor::Editor::canvas(canvas_id);
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
//             editor
//                 .borrow_mut()
//                 .scene
//                 .update_mouse(event.offset_x() as f64, event.offset_y() as f64);
//             editor.borrow_mut().update();
//         }) as Box<dyn FnMut(_)>);
//         canvas
//             .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         //let canvas = canvas.clone();
//         let canvas = editor::Editor::canvas(canvas_id);
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
//             editor.borrow_mut().scene.mouse.mouseup();
//             editor.borrow_mut().update();
//         }) as Box<dyn FnMut(_)>);
//         canvas
//             .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         //let canvas = canvas.clone();
//         let canvas = editor::Editor::canvas(canvas_id);
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
//             editor
//                 .borrow_mut()
//                 .scene
//                 .mouse
//                 .mousedown(event.offset_x() as f64, event.offset_y() as f64);
//             editor.borrow_mut().update();
//         }) as Box<dyn FnMut(_)>);
//         canvas
//             .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         //let canvas = canvas.clone();
//         let canvas = editor::Editor::canvas(canvas_id);
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
//             editor.borrow_mut().scene.update_scale(event.delta_y());
//             editor.borrow_mut().update();
//         }) as Box<dyn FnMut(_)>);
//         canvas
//             .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
//             editor.borrow_mut().scene.keystroke(event.key());
//             editor.borrow_mut().update();
//             if event.key() == "s".to_string() {
//                 editor.borrow().to_spice();
//             }
//             if event.key() == "e".to_string() {
//                 web_sys::window()
//                     .unwrap()
//                     .local_storage()
//                     .unwrap()
//                     .unwrap()
//                     .set_item("circuit", &editor.borrow().export())
//                     .unwrap();
//             }
//         }) as Box<dyn FnMut(_)>);
//         dom::document()
//             .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {
//             let (x_label, y_labels, series) =
//                 plot::parser::parse_spice_output(&event.detail().as_string().unwrap());
//             editor
//                 .borrow_mut()
//                 .plots
//                 .update_data(series, x_label, y_labels);
//         }) as Box<dyn FnMut(_)>);
//         web_sys::window()
//             .unwrap()
//             .add_event_listener_with_callback("ngspice_response", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let canvas = dom::canvas::as_canvas(dom::select(plot::CANVAS_ID));
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
//             editor.borrow_mut().plots.mouse =
//                 Some(Point::new(event.offset_x() as f64, event.offset_y() as f64));
//             editor.borrow_mut().plots.draw();
//         }) as Box<dyn FnMut(_)>);
//         canvas
//             .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let canvas = dom::canvas::as_canvas(dom::select(plot::CANVAS_ID));
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
//             editor.borrow_mut().plots.mouse = None;
//             editor.borrow_mut().plots.draw();
//         }) as Box<dyn FnMut(_)>);
//         canvas
//             .add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
//             for element in dom::select_all("[id^=\"menu__simulations-type-\"]") {
//                 element.set_attribute("class", "hide").unwrap();
//             }
//             dom::select(&format!(
//                 "#menu__simulations-type-{}",
//                 dom::form::select::value_as_string("[name=\"sim__type\"]").unwrap()
//             ))
//             .set_attribute("class", "")
//             .unwrap();
//         }) as Box<dyn FnMut(_)>);
//         dom::select("[name=\"sim__type\"]")
//             .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//
//     // Events for the toolbar
//     {
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |_event: web_sys::CustomEvent| {
//             editor.borrow_mut().toolbar_updated();
//             editor.borrow_mut().plots.resize_canvas();
//         }) as Box<dyn FnMut(_)>);
//         for name in vec!["mouse", "components", "properties", "simulation"].iter() {
//             dom::select(&format!("[name=\"toolbar__{}\"]", name))
//                 .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
//                 .unwrap();
//         }
//         web_sys::window()
//             .unwrap()
//             .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |_: web_sys::MutationRecord| {
//             let editor = editor.clone();
//             let change_closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
//                 editor
//                     .borrow_mut()
//                     .scene
//                     .entities
//                     .update_selected_entity_properties();
//                 editor.borrow_mut().update();
//             }) as Box<dyn FnMut(_)>);
//             for element in dom::select_all("[name^=\"property__\"]").iter() {
//                 element
//                     .add_event_listener_with_callback(
//                         "change",
//                         change_closure.as_ref().unchecked_ref(),
//                     )
//                     .unwrap();
//             }
//             change_closure.forget();
//         }) as Box<dyn FnMut(_)>);
//         let mut config = web_sys::MutationObserverInit::new();
//         config.child_list(true);
//         web_sys::MutationObserver::new(closure.as_ref().unchecked_ref())
//             .unwrap()
//             .observe_with_options(&dom::select("#menu__property-list"), &config)
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
//             dom::select("#error__container")
//                 .set_attribute("class", "hide")
//                 .unwrap();
//         }) as Box<dyn FnMut(_)>);
//         dom::select("#error__close")
//             .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
//             editor.borrow_mut().plots.update_visible_series();
//         }) as Box<dyn FnMut(_)>);
//         dom::select("#sim__result-settings")
//             .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
//             editor.borrow_mut().plots.select();
//         }) as Box<dyn FnMut(_)>);
//         dom::select("#sim__results-selector")
//             .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
//     {
//         let editor = editor.clone();
//         let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
//             editor.borrow_mut().plots.add_plot();
//         }) as Box<dyn FnMut(_)>);
//         dom::select("#sim__results-add")
//             .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
//             .unwrap();
//         closure.forget();
//     }
// }
