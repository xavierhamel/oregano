use crate::intrinsics::*;
use crate::schema::{ctx, mouse};
use crate::{clog, dom, error};

pub struct Scene {
    pub offset: Point,
    pub scale: f64,
    pub size: Size,
}

impl Scene {
    pub fn new() -> Self {
        let mut scene = Self {
            offset: Point::new(0.0, 0.0),
            scale: 2.1,
            size: Size::new(1.0, 1.0),
        };
        let _ = scene.resize();
        scene
    }

    pub fn compute_offset(&mut self, mouse: &mouse::Mouse) {
        if mouse.action == mouse::Action::MoveView {
            let mut diff = mouse.prev_screen_pos - mouse.screen_pos;
            diff.x = diff.x / self.scale;
            diff.y = diff.y / self.scale;
            self.offset = self.offset + diff;
        }
    }

    pub fn compute_scale(&mut self, delta: f64, mouse: &mouse::Mouse) {
        let zoom = 1.0 + (delta / 300.0) / 2.0;
        self.scale *= zoom;
        self.scale = self.scale.max(0.3);
        self.scale = self.scale.min(11.0);
        let new_offset = Point::new(
            mouse.screen_pos.x / self.scale,
            mouse.screen_pos.y / self.scale,
        ) + self.offset;
        self.offset = self.offset + mouse.scene_pos - new_offset;
    }

    pub fn draw_grid(&self, ctx: &ctx::Ctx) {
        for (color, step_by) in [
            ("#111111", Point::GRID_SIZE as usize),
            ("#222222", Point::GRID_SIZE as usize * 5),
        ]
        .iter()
        {
            let offset = Point::new(
                self.offset.x % *step_by as f64,
                self.offset.y % *step_by as f64,
            );
            ctx.begin_path();
            ctx.set_stroke_style(0.5, color);
            for x in (0..((self.size.w / self.scale) as usize)).step_by(*step_by) {
                ctx.move_to(Point::new(x as f64 - offset.x, 0.0));
                ctx.line_to(Point::new(x as f64 - offset.x, self.size.h / self.scale));
            }
            for y in (0..((self.size.h / self.scale) as usize)).step_by(*step_by) {
                ctx.move_to(Point::new(0.0, y as f64 - offset.y));
                ctx.line_to(Point::new(self.size.w / self.scale, y as f64 - offset.y));
            }
            ctx.stroke();
        }
    }

    pub fn resize(&mut self) -> Result<(), error::Error> {
        let container =
            dom::convert::<web_sys::HtmlElement>(dom::select("#menu__editor-container"))?;
        let canvas = dom::convert::<web_sys::HtmlCanvasElement>(dom::select("#canvas"))?;
        canvas.set_width(container.offset_width() as u32);
        canvas.set_height(container.offset_height() as u32);
        self.size = Size::new(
            container.offset_width() as f64,
            container.offset_height() as f64,
        );
        Ok(())
    }
}
