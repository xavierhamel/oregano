pub mod ctx;
pub mod layout;
mod mouse;
pub mod parts;
pub mod properties;
pub mod props;
mod scene;
pub mod wire;

use crate::intrinsics::*;
use crate::schema::parts::part;
use crate::{dom, error, events, views};

pub struct Schema {
    ctx: ctx::Ctx,
    parts: parts::Parts,
    wires: wire::Wires,
    pub mouse: mouse::Mouse,
    scene: scene::Scene,
}

impl Schema {
    pub fn new() -> Result<Self, error::Error> {
        Ok(Self {
            ctx: ctx::Ctx::new(dom::select("#canvas"))?,
            parts: parts::Parts::new(),
            wires: wire::Wires::new(),
            mouse: mouse::Mouse::new(),
            scene: scene::Scene::new(),
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
            _ => {}
        }
        self.update();
    }

    pub fn properties_dispatch(&mut self) {
        self.parts.update_selected();
        self.update();
    }

    pub fn add_wire(&mut self) {
        self.unselect();
        self.mouse.action.set(mouse::Action::DrawWire);
        self.update();
    }

    pub fn add_part(&mut self, part: part::Part) {
        self.parts.add(part);
        self.update();
    }

    pub fn unselect(&mut self) {
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
        self.parts.update(&self.mouse);
        self.wires.update(&mut self.mouse);
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
}
