use crate::intrinsics::*;
use crate::plot;
use wasm_bindgen::JsValue;

/// Represent an axis of a plot
pub struct Axis {
    tick_count: usize,
    pub labels: Vec<String>,
}

impl Axis {
    pub fn new(tick_count: usize, labels: Vec<String>) -> Self {
        Self { tick_count, labels }
    }

    pub fn draw_horizontal_grid(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        min: f64,
        max: f64,
        offset: Point,
        size: Size,
    ) {
        let zero_pos = (1.0 - (max / (max - min))) * (size.w - offset.x) + offset.x;
        let y = size.h + offset.y;
        let spacing = size.w / (self.tick_count - 1) as f64;

        context.set_fill_style(&JsValue::from_str("#CCC"));
        context.set_text_align("center");

        for tick in 0..self.tick_count {
            let x = offset.x + tick as f64 * spacing;
            self.draw_line(x, offset.y, x, y, "#323232", context);
            self.draw_line(x, y, x, y + 5.0, "#CCC", context);
            let tick_label = format!(
                "{:.2}",
                ((max - min) * (tick as f64 / (self.tick_count - 1) as f64)) + min
            );
            context.fill_text(&tick_label, x, y + 17.0).unwrap();
        }
        self.draw_line(zero_pos, offset.y, zero_pos, y, "#555", context);
    }

    pub fn draw_horizontal(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        offset: Point,
        size: Size,
    ) {
        context.set_text_align("center");
        context.set_fill_style(&JsValue::from_str("#CCC"));
        self.labels.iter().enumerate().for_each(|(idx, label)| {
            context
                .fill_text(
                    &label,
                    size.w / 2.0 + offset.x,
                    size.h + offset.y + 30.0 + (idx as f64 * 15.0),
                )
                .unwrap();
        });
        let y = size.h + offset.y;
        self.draw_line(offset.x, y, size.w + offset.x, y, "#CCC", context);
    }

    pub fn draw_vertical_grid(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        min: f64,
        max: f64,
        offset: Point,
        size: Size,
    ) {
        let zero_pos = max / (max - min) * size.h + offset.y;
        let x = offset.x;
        let spacing = size.h / (self.tick_count - 1) as f64;

        context.set_fill_style(&JsValue::from_str("#CCC"));
        context.set_text_align("right");
        self.draw_line(x, offset.y, x, size.h + offset.y, "#CCC", context);

        for tick in 0..self.tick_count {
            let y = size.h + offset.y - tick as f64 * spacing;
            self.draw_line(x, y, size.w + offset.x, y, "#323232", context);
            self.draw_line(x, y, x - 5.0, y, "#CCC", context);
            let tick_label = ((max - min) * (tick as f64 / (self.tick_count - 1) as f64)) + min;
            context
                .fill_text(&format!("{:.2}", tick_label), x - 10.0, y + 5.0)
                .unwrap();
        }
        self.draw_line(x, zero_pos, size.w + offset.x, zero_pos, "#555", context);
    }

    pub fn draw_vertical(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        offset: Point,
        size: Size,
        visible_series: &Vec<bool>,
    ) {
        context.set_text_align("center");
        self.labels.iter().enumerate().for_each(|(idx, label)| {
            if visible_series[idx] {
                context.save();
                context
                    .translate(
                        offset.x - 35.0 - (idx as f64 * 10.0),
                        size.h / 2.0 + offset.y,
                    )
                    .unwrap();
                context.rotate(-1.57).unwrap();
                context.set_fill_style(&JsValue::from_str(plot::COLORS[idx]));
                context.fill_text(&label, 0.0, 0.0).unwrap();
                context.restore();
            }
        });
        self.draw_line(
            offset.x,
            offset.y,
            offset.x,
            size.h + offset.y,
            "#CCC",
            context,
        );
    }

    fn draw_line(
        &self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        style: &str,
        context: &web_sys::CanvasRenderingContext2d,
    ) {
        context.set_stroke_style(&JsValue::from_str(style));
        context.begin_path();
        context.move_to(x1, y1);
        context.line_to(x2, y2);
        context.stroke();
    }
}
