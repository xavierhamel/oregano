pub mod ctx;
pub mod layout;
mod mouse;
pub mod parts;
pub mod properties;
pub mod props;
mod scene;
pub mod shape;
mod utils;
pub mod wires;

use crate::dom::form::{checkbox, radio, select};
use crate::intrinsics::*;
use crate::project;
use crate::schema::parts::part;
use crate::{clog, dom, error, events, sim, sim::circuit};
use wasm_bindgen::JsValue;

pub struct Schema {
    ctx: ctx::Ctx,
    parts: parts::Parts,
    wires: wires::Wires,
    pub mouse: mouse::Mouse,
    scene: scene::Scene,
    pub probes: sim::Probes,
}

impl Schema {
    pub fn new() -> Result<Self, error::Error> {
        Ok(Self {
            ctx: ctx::Ctx::new(dom::select("#canvas"))?,
            parts: parts::Parts::new(),
            wires: wires::Wires::new(),
            mouse: mouse::Mouse::new(),
            scene: scene::Scene::new(),
            probes: sim::Probes::new(vec![], vec![]),
        })
    }

    pub fn mouse_dispatch(&mut self, mouse: Point, event: events::Event) {
        self.mouse.update(mouse, &self.scene, event);
        self.update();
        self.scene.compute_offset(&self.mouse);
    }

    pub fn wheel_dispatch(&mut self, delta: f64) {
        self.scene.compute_scale(delta, &self.mouse);
        self.update();
    }

    pub fn keyboard_dispatch(&mut self, key: &str) {
        match key {
            "w" => self.add_wire(),
            "r" => self.parts.rotate(),
            "Escape" => self.unselect(),
            "Delete" => self.delete(),
            "s" => self.to_spice(),
            "e" => self.export(),
            "i" => self.import(),
            _ => {}
        }
        self.update();
    }

    pub fn properties_dispatch(&mut self) {
        self.parts.update_selected();
        self.update();
    }

    pub fn resize_dispatch(&mut self) {
        let _ = self.scene.resize();
        self.update();
    }

    pub fn toolbar_dispatch(&mut self) {
        match radio::value::<utils::ToolbarTool>(dom::select_all("[name=\"toolbar__mouse\"]")) {
            Ok(utils::ToolbarTool::Wire) => self.add_wire(),
            Ok(utils::ToolbarTool::Mouse) => self.unselect(),
            _ => {}
        }

        for name in ["components", "properties", "simulation", "editor"] {
            let container = format!("[data-toolbar=\"toolbar__{}\"]", name);
            let checkbox = format!("[name=\"toolbar__{}\"]", name);
            if let Ok(true) = checkbox::value(dom::select(&checkbox)) {
                let _ = dom::select(&container).class_list().remove_1("hide");
            } else {
                let _ = dom::select(&container).class_list().add_1("hide");
            }
        }
        if let Ok(false) = checkbox::value(dom::select("[name=\"toolbar__simulation\"]")) {
            let _ = dom::select("[data-toolbar=\"toolbar__editor\"]")
                .class_list()
                .remove_1("hide");
        }
        self.resize_dispatch();
    }

    pub fn simulation_selector_dispatch(&self) {
        if let Ok(typ) = select::value::<String>(dom::select("[name=\"sim__type\"]")) {
            dom::select_all("[id^=\"menu__simulations-type-\"]")
                .iter()
                .for_each(|e| {
                    let _ = e.set_attribute("class", "hide");
                });
            let _ =
                dom::select(&format!("#menu__simulations-type-{}", typ)).set_attribute("class", "");
        }
    }

    pub fn add_wire(&mut self) {
        self.wires.unselect(&mut self.mouse);
        self.parts.unselect();
        self.mouse.action.set_protected(mouse::Action::DrawWire);
        self.update();
    }

    pub fn add_part(&mut self, part: Result<part::Part, error::Error>) {
        if let Ok(part) = part {
            self.parts.add(part);
            self.update();
        }
    }

    pub fn unselect(&mut self) {
        self.mouse.action.set(mouse::Action::None);
        self.wires.unselect(&mut self.mouse);
        self.parts.unselect();
        self.update();
    }

    pub fn delete(&mut self) {
        self.parts.delete();
        self.wires.delete();
        self.update();
    }

    pub fn update(&mut self) {
        self.parts.update(&mut self.mouse);
        self.wires.update(&mut self.mouse, &self.parts);
        // c'est ici qu'on devrait changer l'action si jamais on a un element select.
        self.mouse.update_action(&self.parts, &self.wires);
        self.draw();
    }

    pub fn draw(&mut self) {
        self.ctx.set_fill_style("#000000");
        self.ctx.fill_rect(Point::new(0.0, 0.0), self.scene.size);
        self.ctx.scale(self.scene.scale);
        self.scene.draw_grid(&self.ctx);
        self.ctx.translate(-self.scene.offset); // + Point::new(0.5, 0.5));
        self.ctx.set_stroke_round();
        self.parts.iter().for_each(|part| part.draw(&self.ctx));
        self.wires.iter().for_each(|wire| wire.draw(&self.ctx));

        self.ctx.translate(self.scene.offset); // - Point::new(0.5, 0.5));
        self.ctx.scale(1.0 / self.scene.scale)
    }

    /// This function take the circuit, convert it to a string that can be interpreted by ngspice
    /// (with the `src/simulation/circuit.rs` mod) and finally send it to ngspice (via a nodejs
    /// server). Ngspice will respond back and trigger an event (in `src/events.rs`).
    pub fn to_spice(&mut self) {
        let circuit =
            match circuit::Circuit::new(self.wires.wires.clone(), self.parts.parts.clone()) {
                Ok(circuit) => circuit,
                Err(error) => {
                    error::show_multiple(error);
                    return;
                }
            };
        let (spice_string, probes) = match circuit.to_string() {
            Ok(circuit) => circuit,
            Err(error) => {
                error::show(error);
                return;
            }
        };
        self.probes = probes;

        clog!("{}", spice_string);

        let mut event_data = web_sys::CustomEventInit::new();
        event_data.detail(&JsValue::from_str(&spice_string));
        let event =
            web_sys::CustomEvent::new_with_event_init_dict("ngspice_call", &event_data).unwrap();
        web_sys::window().unwrap().dispatch_event(&event).unwrap();
    }

    pub fn export(&self) {
        let export = project::export::to_oregano(&self.wires.wires, &self.parts.parts);

        web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item("circuit", &export)
            .unwrap();
    }

    pub fn import(&mut self) {
        let input = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("circuit")
            .unwrap()
            .unwrap();
        match project::import::from_oregano(&input) {
            Ok((wires, parts)) => {
                self.wires.wires = wires;
                self.parts.parts = parts;
            }
            Err(error) => error::show(error),
        }
    }
}
