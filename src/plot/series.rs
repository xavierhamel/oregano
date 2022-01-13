use crate::intrinsics::*;
use crate::plot;
use wasm_bindgen::JsValue;

pub struct Series {
    series: Vec<Vec<Point>>,
    is_visible: Vec<bool>,
    pub min: Point,
    pub max: Point,
    scale: Point,
    size: Size,
    offset: Point,
}

impl Series {
    pub fn new(series: Vec<Vec<Point>>, size: Size, offset: Point) -> Self {
        let is_visible = (0..series.len()).map(|_| true).collect::<Vec<bool>>();
        let mut series = Self {
            series,
            is_visible,
            min: Point::new(0.0, 0.0),
            max: Point::new(0.0, 0.0),
            scale: Point::new(1.0, 1.0),
            size,
            offset,
        };
        series.find_extrema();
        series.find_scale();
        series
    }

    /// Find the extrema of all the series together. We need this for the scale of the plot to have
    /// it centered in the canvas.
    pub fn find_extrema(&mut self) {
        let extrema = self.series.iter().enumerate().fold(
            None,
            |accumulator: Option<(Point, Point)>, (idx, series)| {
                if self.is_visible[idx] {
                    series.iter().fold(accumulator, |acc, point| {
                        if let Some(mut extrema) = acc {
                            if point.x < extrema.0.x {
                                extrema.0.x = point.x;
                            }
                            if point.x > extrema.1.x {
                                extrema.1.x = point.x;
                            }
                            if point.y < extrema.0.y {
                                extrema.0.y = point.y;
                            }
                            if point.y > extrema.1.y {
                                extrema.1.y = point.y;
                            }
                            Some(extrema)
                        } else {
                            Some((*point, *point))
                        }
                    })
                } else {
                    accumulator
                }
            },
        );
        if let Some(e) = extrema {
            self.min = e.0;
            self.max = e.1;
        }
    }

    pub fn find_scale(&mut self) {
        self.scale = Point::new(
            self.size.w / (self.max.x - self.min.x),
            self.size.h / (self.max.y - self.min.y),
        );
    }

    /// Draw a series of point to the graph. When drawing the graph, we have to remember that there
    /// is a margin on each side so that no text or line is cut off.
    pub fn draw_series(&self, context: &web_sys::CanvasRenderingContext2d, mouse: &Option<Point>) {
        let mut selected_points = Vec::new();
        for (idx, series) in self.series.iter().enumerate() {
            if series.len() == 0 {
                continue;
            }
            context.begin_path();
            context.set_stroke_style(&JsValue::from_str(plot::COLORS[idx % 8]));
            context.move_to(
                self.offset.x + (series[0].x - self.min.x) * self.scale.x,
                self.offset.y + (self.max.y - series[0].y) * self.scale.y,
            );
            let mut selected_point = None;
            for point in series.iter() {
                let x = self.offset.x + (point.x - self.min.x) * self.scale.x;
                let y = self.offset.y + (self.max.y - point.y) * self.scale.y;
                if let Some(mouse_position) = mouse {
                    if x - 1.0 < mouse_position.x && x + 1.0 > mouse_position.x {
                        selected_point = Some((Point::new(x, y), point));
                    }
                }
                context.line_to(x, y);
            }
            selected_points.push(selected_point);
            context.stroke();
        }
        context.set_text_align("right");
        selected_points
            .iter()
            .enumerate()
            .for_each(|(idx, selected_point)| {
                if let Some((curve_position, point)) = selected_point {
                    context.begin_path();
                    context
                        .arc(curve_position.x, curve_position.y, 3.0, 0.0, 6.28)
                        .unwrap();
                    context.set_stroke_style(&JsValue::from_str("#323232"));
                    context.set_fill_style(&JsValue::from_str(plot::COLORS[idx % 8]));
                    context.stroke();
                    context.fill();
                    context
                        .fill_text(
                            &format!("({:.2}, {:.2})", point.x, point.y),
                            self.size.w + self.offset.x - 3.0,
                            self.offset.y + (idx as f64 * 15.0),
                        )
                        .unwrap();
                }
            });
    }
}
