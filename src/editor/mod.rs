use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::simulation::circuit;
use crate::log;
use crate::dom;

pub mod scene;
pub mod entity;
mod entities;
pub mod styles;
pub mod shape;
pub mod component;
pub mod wire;
pub mod mouse;
pub mod components;

pub struct Editor {
    pub scene: scene::Scene,
}

impl Editor {
    pub fn new(canvas_id: &str) -> Self {
        let mut scene = scene::Scene::new(canvas_id);
        scene.update();
        scene.entities.update_connections();
        scene.update_size();
        Self {
            scene,
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
    
    pub fn to_spice(&self) {
        let components = self.scene.components();
        let wires = self.scene.wires();

        let circuit = circuit::Circuit::new(wires, components).unwrap();
        let spice = circuit.to_string().unwrap();
        log(&spice);
        self.spice_server(&spice);
    }

    pub fn spice_server(&self, circuit: &str) {
        let mut event_data = web_sys::CustomEventInit::new();
        event_data.detail(&JsValue::from_str(circuit));
        let event = web_sys::CustomEvent::new_with_event_init_dict("ngspice_call", &event_data).unwrap();
        web_sys::window().unwrap().dispatch_event(&event);
        log("dispatched");
    }

    pub fn toolbar_updated(&mut self) {
        let hide_class = "hide";
        for name in vec!["components", "properties", "simulation"].iter() {
            let selector = format!("[name=\"toolbar__{}\"]", name);
            let container_selector = format!("[data-toolbar=\"toolbar__{}\"]", name);
            if Ok(true) == dom::form::checkbox::value_as_bool(&selector) {
                dom::select(&container_selector).class_list().remove_1(hide_class);
            } else {
                dom::select(&container_selector).class_list().add_1(hide_class);
            }
        }
        self.scene.update_size();
        self.update();
    }
}
