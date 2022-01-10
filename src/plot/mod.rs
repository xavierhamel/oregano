use crate::intrinsics::*;
use crate::dom;
use wasm_bindgen::JsValue;
use crate::log;


const AXIS_WIDTH: f64 = 35.0;

/// Represent an axis of a plot
struct Axis {
    label: String,
    tick_count: usize,
    min: f64,
    max: f64,
}

impl Axis {
    pub fn new(label: String, min: f64, max: f64, tick_count: usize) -> Self {
        Self {
            label,
            min,
            max,
            tick_count,
        }
    }

    pub fn scale(&self, size: f64) -> f64 {
        size / (self.max - self.min)
    }

    pub fn draw_horizontal(&self, context: &web_sys::CanvasRenderingContext2d, canvas_size: &Size) {
        let y = canvas_size.h - AXIS_WIDTH;
        context.set_stroke_style(&JsValue::from_str("#CCC"));
        context.set_fill_style(&JsValue::from_str("#CCC"));
        context.set_text_align("center");
        context.begin_path();
        context.move_to(AXIS_WIDTH, y);
        context.line_to(canvas_size.w, y);
        for tick in 0..self.tick_count {
            let spacing = (canvas_size.w - AXIS_WIDTH) / self.tick_count as f64;
            let x = AXIS_WIDTH + tick as f64 * spacing;
            context.move_to(x, y);
            context.line_to(x, y + 5.0);
            let tick_label = format!("{:.2}", ((self.max - self.min) * (tick as f64 / (self.tick_count) as f64)));
            context.fill_text(&tick_label, x, y + 17.0).unwrap();
        }
        context.stroke();
    }

    pub fn draw_vertical(&self, context: &web_sys::CanvasRenderingContext2d, canvas_size: &Size) {
        let x = AXIS_WIDTH;
        context.set_stroke_style(&JsValue::from_str("#CCC"));
        context.set_fill_style(&JsValue::from_str("#CCC"));
        context.set_text_align("right");
        context.begin_path();
        context.move_to(x, 0.0);
        context.line_to(x, canvas_size.h - AXIS_WIDTH);
        for tick in 0..self.tick_count {
            let spacing = (canvas_size.h - AXIS_WIDTH) / self.tick_count as f64;
            let y = canvas_size.h - AXIS_WIDTH - tick as f64 * spacing;
            context.move_to(x, y);
            context.line_to(x - 5.0, y);
            let tick_label = format!("{:.2}", ((self.max - self.min) * (tick as f64 / (self.tick_count) as f64)));
            context.fill_text(&tick_label, x - 10.0, y + 5.0).unwrap();
        }
        context.stroke();
    }
}

/// Represent a plot
pub struct Plot {
    points: Vec<Point>,
    x_axis: Axis,
    y_axis: Axis,
    color: Color,
    selector: String,
}

impl Plot {
    pub fn new(
        mut points: Vec<Point>,
        x_label: String,
        y_label: String,
        color: Color,
        selector: String
    ) -> Self {
        if points.len() == 0 {
            points.push(Point::new(0.0, 0.0));
        }
        let x_axis = Axis::new(
            x_label,
            points[0].x,
            points[points.len() - 1].x,
            5,
        );
        let (min, max) = min_max_of_y(&points);
        let y_axis = Axis::new(
            y_label,
            min,
            max,
            5,
        );
        Self {
            points,
            x_axis,
            y_axis,
            color,
            selector,
        }
    }

    fn size(&self) -> Size {
        let canvas = dom::canvas::as_canvas(dom::select(&self.selector));
        Size::new(
            canvas.get_attribute("width").unwrap().parse::<usize>().unwrap() as f64,
            canvas.get_attribute("height").unwrap().parse::<usize>().unwrap() as f64,
        )
    }

    pub fn draw(&self) {
        let context = dom::canvas::context(
            &dom::canvas::as_canvas(dom::select(&self.selector))
        );
        let size = self.size();
        let x_scale = self.x_axis.scale(size.w - AXIS_WIDTH);
        let y_scale = self.y_axis.scale(size.h - AXIS_WIDTH);

        context.clear_rect(0.0, 0.0, size.w, size.h);
        context.begin_path();
        context.set_stroke_style(&JsValue::from_str(self.color.0));
        context.move_to(AXIS_WIDTH + self.points[0].x * x_scale, self.points[0].y * y_scale);
        for point in &self.points {
            context.line_to(AXIS_WIDTH + point.x * x_scale, (size.h - AXIS_WIDTH) - point.y * y_scale);
        }
        context.stroke();
        self.x_axis.draw_horizontal(&context, &size);
        self.y_axis.draw_vertical(&context, &size);
    }
}

pub fn parse_spice_output(output: String) -> Vec<Point> {
    let mut points = Vec::new();
    output.lines().for_each(|line| {
        let cols = line.split_whitespace().collect::<Vec<&str>>();
        // If we have 3 columns and the first one can be represented as a usize, we have a data
        // line.
        if cols.len() == 3 && cols[0].parse::<usize>().is_ok() {
            if let (Ok(x), Ok(y)) = (cols[1].parse::<f64>(), cols[2].parse::<f64>()) {
                points.push(Point::new(x, y));
            }
        }
    });
    points
}

fn min_max_of_y(points: &Vec<Point>) -> (f64, f64) {
    let mut min = points[0].y;
    let mut max = points[0].y;
    for point in points {
        if point.y < min {
            min = point.y;
        }
        if point.y > max {
            max = point.y;
        }
    }
    (min, max)
}
