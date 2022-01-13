use crate::dom;
use crate::log;
use crate::plot;
use crate::simulation::circuit;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod component;
mod entities;
pub mod entity;
pub mod mouse;
pub mod property;
pub mod scene;
pub mod shape;
pub mod styles;
pub mod wire;

pub struct Editor {
    pub scene: scene::Scene,
    pub plots: plot::Plots,
}

impl Editor {
    pub fn new(canvas_id: &str) -> Self {
        let mut scene = scene::Scene::new(canvas_id);
        scene.update();
        scene.entities.update_connections();
        scene.update_size();
        Self {
            scene,
            plots: plot::Plots::new(),
        }
    }

    pub fn update(&mut self) {
        self.scene.update();
        self.scene.draw();
    }

    pub fn canvas(canvas_id: &str) -> web_sys::HtmlCanvasElement {
        // TODO: We need to add some error checking for this section.
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas
    }

    /// This function take the circuit, convert it to a string that can be interpreted by ngspice
    /// (with the `src/simulation/circuit.rs` mod) and finally send it to ngspice (via a nodejs
    /// server). Ngspice will respond back and trigger an event (in `src/events.rs`).
    pub fn to_spice(&self) {
        let components = self.scene.components();
        let wires = self.scene.wires();

        let circuit = match circuit::Circuit::new(wires, components) {
            Ok(circuit) => circuit,
            Err(error) => {
                error.show();
                return ();
            }
        };
        let spice_string = match circuit.to_string() {
            Ok(circuit) => circuit,
            Err(error) => {
                error.show();
                return ();
            }
        };

        log(&spice_string);

        let mut event_data = web_sys::CustomEventInit::new();
        event_data.detail(&JsValue::from_str(&spice_string));
        let event =
            web_sys::CustomEvent::new_with_event_init_dict("ngspice_call", &event_data).unwrap();
        web_sys::window().unwrap().dispatch_event(&event).unwrap();
    }

    pub fn toolbar_updated(&mut self) {
        let hide_class = "hide";
        for name in vec!["components", "properties", "simulation"].iter() {
            let selector = format!("[name=\"toolbar__{}\"]", name);
            let container_selector = format!("[data-toolbar=\"toolbar__{}\"]", name);
            if Ok(true) == dom::form::checkbox::value_as_bool(&selector) {
                dom::select(&container_selector)
                    .class_list()
                    .remove_1(hide_class)
                    .unwrap();
            } else {
                dom::select(&container_selector)
                    .class_list()
                    .add_1(hide_class)
                    .unwrap();
            }
        }
        self.scene.update_size();
        self.update();
    }
}
