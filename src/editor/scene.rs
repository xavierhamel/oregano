use crate::dom;
use crate::editor::{component, entities, mouse, wire};
use crate::intrinsics::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys;

pub struct Scene {
    height: usize,
    width: usize,
    offset: Point,
    scale: f64,
    context: web_sys::CanvasRenderingContext2d,
    pub canvas: web_sys::HtmlCanvasElement,
    pub mouse: mouse::Mouse,
    pub entities: entities::Entities,
}

impl Scene {
    pub fn new(canvas_id: &str) -> Self {
        let (canvas, context) = Scene::context(canvas_id);

        Self {
            width: 940,
            height: 730,
            offset: Point::new(0.0, 0.0),
            scale: 1.5,
            canvas,
            context,
            mouse: mouse::Mouse::new(),
            entities: entities::Entities::new(),
        }
    }

    pub fn update_mouse(&mut self, x: f64, y: f64) {
        self.mouse.update(x, y, self.offset, self.scale);
        self.entities.update_hovered(&self.mouse);
        self.compute_offset();
    }

    pub fn update_scale(&mut self, delta_y: f64) {
        self.scale += delta_y * -0.001;
        if self.scale < 0.3 {
            self.scale = 0.3;
        }
    }

    fn compute_offset(&mut self) {
        if self.mouse.action == mouse::MouseAction::MoveView {
            let mut diff = self.mouse.prev_screen_pos - self.mouse.screen_pos;
            diff.x = diff.x / self.scale;
            diff.y = diff.y / self.scale;
            self.offset = self.offset + diff;
        }
    }

    pub fn update(&mut self) {
        match self.mouse.state {
            mouse::MouseState::Down => {
                if self.entities.floating_component.is_some() {
                    self.entities.add_component();
                    self.mouse.action = mouse::MouseAction::None;
                } else {
                    self.entities.select(&mut self.mouse);
                }
            }
            mouse::MouseState::Drag => {
                if self.mouse.action == mouse::MouseAction::MoveEntity {
                    self.entities.drag(&self.mouse)
                }
            }
            mouse::MouseState::Click => {
                if !self.entities.were_some_selected {
                    self.entities.add_wire(self.mouse.scene_pos);
                }
                self.mouse.state = mouse::MouseState::Up;
            }
            _ => {
                self.entities.update_floating_wire(&self.mouse);
                self.entities.update_floating_component(&self.mouse);
            }
        }
    }

    pub fn draw(&self) {
        self.context
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        self.context.set_fill_style(&JsValue::from_str("#000000"));
        self.context
            .fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
        self.context.scale(self.scale, self.scale).unwrap();
        self.context.set_fill_style(&JsValue::from_str("#111111"));
        let grid_offset_x = self.offset.x % 10.0;
        let grid_offset_y = self.offset.y % 10.0;
        for x in (0..self.width).step_by(10) {
            for y in (0..self.height).step_by(10) {
                self.context.fill_rect(
                    x as f64 - grid_offset_x,
                    y as f64 - grid_offset_y,
                    1.0,
                    1.0,
                );
            }
        }

        self.context
            .translate(-self.offset.x + 0.5, -self.offset.y + 0.5)
            .unwrap();
        self.entities.draw(&self.context, &self.mouse);
        self.context
            .translate(self.offset.x - 0.5, self.offset.y - 0.5)
            .unwrap();
        self.context
            .scale(1.0 / self.scale, 1.0 / self.scale)
            .unwrap();
    }

    pub fn keystroke(&mut self, key: String) {
        match key.as_str() {
            "r" => self.entities.rotate_selected(),
            "Delete" => self.entities.delete_selected(&mut self.mouse),
            "Escape" => self.entities.unselect_all(),
            _ => {}
        }
    }

    fn context(
        canvas_id: &str,
    ) -> (
        web_sys::HtmlCanvasElement,
        web_sys::CanvasRenderingContext2d,
    ) {
        // TODO: We need to add some error checking for this section.
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        (canvas, context)
    }

    pub fn components(&self) -> Vec<component::Component> {
        let mut components: Vec<component::Component> = Vec::new();
        for entity in &self.entities.entities {
            if !entity.is_wire() {
                if let Some(component) = entity.as_any().downcast_ref::<component::Component>() {
                    components.push(component.clone());
                }
            }
        }
        components
    }

    pub fn wires(&self) -> Vec<wire::Wire> {
        let mut wires = Vec::new();
        for entity in &self.entities.entities {
            if entity.is_wire() {
                if let Some(wire) = entity.as_any().downcast_ref::<wire::Wire>() {
                    wires.push(wire.clone());
                }
            }
        }
        wires
    }

    pub fn update_size(&mut self) {
        let container = dom::select("#menu__editor-container")
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| ())
            .unwrap();
        self.canvas.set_width(container.offset_width() as u32);
        self.canvas.set_height(container.offset_height() as u32);
        self.width = container.offset_width() as usize;
        self.height = container.offset_height() as usize;
    }
}
