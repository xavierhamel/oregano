use crate::intrinsics::*;
use crate::schema::layout::shape;
use crate::{dom, error};
use wasm_bindgen::{JsCast, JsValue};

pub struct Ctx {
    ctx: web_sys::CanvasRenderingContext2d,
    scale: f64,
}

impl Ctx {
    pub fn new(canvas: web_sys::Element) -> Result<Self, error::Error> {
        let canvas = dom::convert::<web_sys::HtmlCanvasElement>(canvas)?;
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        Ok(Self { ctx, scale: 1.0 })
    }

    pub fn set_stroke_round(&self) {
        self.ctx.set_line_cap("round");
        self.ctx.set_line_join("round");
    }

    pub fn scale(&mut self, s: f64) {
        self.scale = s;
        let _ = self.ctx.scale(s, s);
    }

    pub fn translate(&self, offset: Point) {
        let _ = self.ctx.translate(offset.x, offset.y);
    }

    pub fn begin_path(&self) {
        self.ctx.begin_path();
    }

    pub fn set_fill_style(&self, color: &str) {
        self.ctx.set_fill_style(&JsValue::from_str(color));
    }

    pub fn fill_arc(&self, origin: Point, arc: &shape::Arc) {
        self.ctx.begin_path();
        arc.add_path(origin, &self);
        self.ctx.fill();
    }

    pub fn fill_rect(&self, origin: Point, size: Size) {
        self.ctx.fill_rect(origin.x, origin.y, size.w, size.h);
    }

    pub fn stroke_shape(&self, origin: Point, shape: &shape::Shape) {
        self.ctx.begin_path();
        shape.add_path(origin, &self);
        self.ctx.stroke();
    }

    pub fn stroke_poly(&self, origin: Point, poly: &shape::Poly) {
        self.ctx.begin_path();
        poly.add_path(origin, &self);
        self.ctx.stroke();
    }

    pub fn stroke_points(&self, points: &[Point]) {
        if points.len() > 0 {
            self.begin_path();
            self.move_to(points[0]);
            points.iter().for_each(|point| self.line_to(*point));
            self.stroke();
        }
    }

    pub fn stroke_arc(&self, origin: Point, arc: &shape::Arc) {
        self.ctx.begin_path();
        arc.add_path(origin, &self);
        self.ctx.stroke();
    }

    pub fn stroke(&self) {
        self.ctx.stroke();
    }

    pub fn stroke_rect(&self, origin: Point, size: Size) {
        self.ctx.stroke_rect(origin.x, origin.y, size.w, size.h);
    }

    pub fn move_to(&self, point: Point) {
        self.ctx.move_to(point.x, point.y);
    }

    pub fn line_to(&self, point: Point) {
        self.ctx.line_to(point.x, point.y);
    }

    pub fn arc(&self, center: Point, radius: f64, start: f64, end: f64) {
        let _ = self.ctx.arc(
            center.x,
            center.y,
            radius,
            start / 180.0 * std::f64::consts::PI,
            end / 180.0 * std::f64::consts::PI,
        );
    }

    pub fn set_stroke_style(&self, line_width: f64, color: &str) {
        self.ctx.set_line_width(line_width);
        self.ctx.set_stroke_style(&JsValue::from_str(color));
    }

    /// This will set the stroke style and the line width will be the same no matter the scale of
    /// the current screen.
    pub fn set_stroke_style_const(&self, line_width: f64, color: &str) {
        self.set_stroke_style(line_width / self.scale, color);
    }

    pub fn set_line_dash(&self, line_dash: Vec<f64>) {
        let dash_array = js_sys::Array::new_with_length(line_dash.len() as u32);
        line_dash.iter().enumerate().for_each(|(idx, dash_len)| {
            dash_array.set(idx as u32, JsValue::from_f64(*dash_len));
        });
        let _ = self.ctx.set_line_dash(&dash_array);
    }

    pub fn set_line_dash_const(&self, line_dash: Vec<f64>) {
        let dash_array = js_sys::Array::new_with_length(line_dash.len() as u32);
        line_dash.iter().enumerate().for_each(|(idx, dash_len)| {
            dash_array.set(idx as u32, JsValue::from_f64(*dash_len / self.scale));
        });
        let _ = self.ctx.set_line_dash(&dash_array);
    }
}
