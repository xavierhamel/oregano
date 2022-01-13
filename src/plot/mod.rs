mod axis;
pub mod parser;
pub mod test;
use crate::dom;
use crate::intrinsics::*;
use wasm_bindgen::{JsCast, JsValue};

const AXIS_WIDTH: f64 = 45.0;
pub const CANVAS_ID: &'static str = "#simulation__canvas";
pub const COLORS: [&'static str; 8] = [
    "#1abc9c", "#e74c3c", "#3498db", "#9b59b6", "#2ecc71", "#f1c40f", "#e67e22", "#95a5a6",
];

/// Contains all the plots created by the user including which one is currently selected. `Plots`
/// will manage the selected plot, show the correct settings when switching to an other plot and
/// redrawing the correct plot to the canvas when switching plot.
pub struct Plots {
    series: Vec<Vec<Point>>,
    plots: Vec<Plot2>,
    selected_idx: Option<usize>,
    size: Size,
    offset: Point,
    pub mouse: Option<Point>,
    x_axis: axis::Axis,
    y_axis: axis::Axis,
    plot_count: usize,
}

impl Plots {
    pub fn new() -> Self {
        //+ y_labels.len() as f64 * 10.0
        let offset = Point::new(AXIS_WIDTH + 12.0, 10.0);
        let size = dom::canvas::size(CANVAS_ID) - Size::new(offset.x + 22.0, AXIS_WIDTH + 10.0);
        let mut plots = Self {
            size,
            offset,
            series: Vec::new(),
            plots: Vec::new(),
            selected_idx: None,
            mouse: None,
            x_axis: axis::Axis::new(5, Vec::new()),
            y_axis: axis::Axis::new(5, Vec::new()),
            plot_count: 0,
        };
        plots.resize_canvas();
        //plots.add_plot();
        plots.update_selector();
        plots
    }

    pub fn add_plot(&mut self) {
        self.plots.push(Plot2::new(
            (0..self.series.len()).map(|_| true).collect::<Vec<bool>>(),
            &self.series,
            &self.size,
        ));
        let id: &str = &format!("sim__result-{}", self.plot_count);
        let label: &str = &format!("Plot {}", self.plot_count + 1);
        let plot_idx = &self.plot_count.to_string();
        dom::append_children(
            &dom::select("#sim__results-selector"),
            vec![
                &dom::create_element(
                    "input",
                    dom::attributes! {
                        "type" => "radio",
                        "name" => "sim__result-selector",
                        "id" => &id,
                        "value" => plot_idx,
                    },
                    vec![],
                ),
                &dom::create_element(
                    "label",
                    dom::attributes! {
                        "inner_html" => label,
                        "for" => id,
                    },
                    vec![],
                ),
            ],
        );
        if self.selected_idx.is_none() {
            self.selected_idx = Some(0);
            dom::select("[name=\"sim__result-selector\"]")
                .dyn_into::<web_sys::HtmlInputElement>()
                .map_err(|_| ())
                .unwrap()
                .set_checked(true);
        }
        self.draw();
        self.plot_count += 1;
    }

    pub fn select(&mut self) {
        for element in dom::select_all("[name=\"sim__result-selector\"]") {
            let radio = element
                .dyn_into::<web_sys::HtmlInputElement>()
                .map_err(|_| ())
                .unwrap();
            if radio.checked() {
                if let Ok(idx) = radio.value().parse::<usize>() {
                    self.selected_idx = Some(idx);
                }
            }
        }
        if let Some(idx) = self.selected_idx {
            self.plots[idx]
                .visible_series
                .iter()
                .enumerate()
                .for_each(|(label_idx, series)| {
                    let element = dom::select(&format!(
                        "[data-series=\"{}\"]",
                        self.y_axis.labels[label_idx]
                    ))
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .map_err(|_| ())
                    .unwrap();
                    element.set_checked(*series);
                });
        }
        self.update_visible_series();
        self.draw();
    }

    pub fn resize_canvas(&mut self) {
        let canvas = dom::canvas::as_canvas(dom::select(CANVAS_ID));
        let container = dom::select("#simulations__canvas-container")
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| ())
            .unwrap();
        canvas.set_width(container.offset_width() as u32);
        canvas.set_height(container.offset_height() as u32);
        self.size = dom::canvas::size(CANVAS_ID) - Size::new(AXIS_WIDTH + 34.0, AXIS_WIDTH + 10.0);
        self.draw();
    }

    pub fn update_data(&mut self, series: Vec<Vec<Point>>, x_label: String, y_labels: Vec<String>) {
        let mut series_map = Vec::new();
        'outer: for label in self.y_axis.labels.iter() {
            for (idx, new_label) in y_labels.iter().enumerate() {
                if label == new_label {
                    series_map.push(Some(idx));
                    continue 'outer;
                }
            }
            series_map.push(None);
        }
        self.plots.iter_mut().for_each(|plot| {
            plot.update_visible_series(&series_map, y_labels.len());
        });
        self.series = series;
        self.x_axis.labels = vec![x_label];
        self.y_axis.labels = y_labels;
        self.draw();
        self.update_selector();
    }

    pub fn update_selector(&mut self) {
        let container = dom::select("#sim__result-settings");
        container.set_inner_html("");
        self.y_axis.labels.iter().for_each(|label| {
            dom::append_children(
                &container,
                vec![&dom::form::group(vec![
                    dom::create_element(
                        "input",
                        dom::attributes! {
                            "type" => "checkbox",
                            "name" => "sim__result-settings-series",
                            "checked" => "",
                            "data-series" => label,
                        },
                        vec![],
                    ),
                    dom::form::label::create(&label),
                ])],
            );
        });
        self.resize_canvas();
    }

    pub fn update_visible_series(&mut self) {
        if let Some(selected_idx) = self.selected_idx {
            let mut visible_series = (0..self.series.len()).map(|_| false).collect::<Vec<bool>>();
            for element in dom::select_all("[name=\"sim__result-settings-series\"]") {
                let checkbox = element
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .map_err(|_| ())
                    .unwrap();
                if checkbox.checked() {
                    for (idx, label) in self.y_axis.labels.iter().enumerate() {
                        if let Some(series) = checkbox.get_attribute("data-series") {
                            if &series == label {
                                visible_series[idx] = true;
                            }
                        }
                    }
                }
            }
            self.plots[selected_idx].visible_series = visible_series;
        }
        self.draw();
    }

    pub fn draw(&mut self) {
        let context = dom::canvas::context(&dom::canvas::as_canvas(dom::select(CANVAS_ID)));
        context.clear_rect(0.0, 0.0, self.size.w * 2.0, self.size.h * 2.0);
        if let Some(idx) = self.selected_idx {
            let plot = &mut self.plots[idx];
            let size = self.size - Size::new(plot.offset.x, 0.0);
            plot.find_extrema(&self.series);
            plot.find_scale(size);
            self.x_axis.draw_horizontal_grid(
                &context,
                plot.min.x,
                plot.max.x,
                plot.offset + self.offset,
                size,
            );
            self.y_axis.draw_vertical_grid(
                &context,
                plot.min.y,
                plot.max.y,
                plot.offset + self.offset,
                size,
            );
            plot.draw(&self.series, &context, &self.mouse, &size, self.offset);
            self.x_axis
                .draw_horizontal(&context, plot.offset + self.offset, size);
            self.y_axis.draw_vertical(
                &context,
                plot.offset + self.offset,
                size,
                &plot.visible_series,
            );
        }
    }
}

pub struct Plot2 {
    visible_series: Vec<bool>,
    min: Point,
    max: Point,
    scale: Point,
    offset: Point,
}

impl Plot2 {
    pub fn new(visible_series: Vec<bool>, series: &Vec<Vec<Point>>, size: &Size) -> Self {
        let series_count = visible_series.iter().filter(|s| **s).count();
        let mut plot = Self {
            visible_series,
            min: Point::new(0.0, 0.0),
            max: Point::new(0.0, 0.0),
            scale: Point::new(1.0, 1.0),
            offset: Point::new(10.0 * series_count as f64, 0.0),
        };
        plot.find_extrema(series);
        plot.find_scale(*size);
        plot
    }

    /// Find the extrema of all the series together. We need this for the scale of the plot to have
    /// it centered in the canvas.
    pub fn find_extrema(&mut self, series: &Vec<Vec<Point>>) {
        let extrema = series.iter().enumerate().fold(
            None,
            |accumulator: Option<(Point, Point)>, (idx, s)| {
                if self.visible_series[idx] {
                    s.iter().fold(accumulator, |acc, point| {
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

    pub fn find_scale(&mut self, size: Size) {
        self.scale = Point::new(
            size.w / (self.max.x - self.min.x),
            size.h / (self.max.y - self.min.y),
        );
    }

    pub fn update_visible_series(&mut self, map: &Vec<Option<usize>>, series_count: usize) {
        let mut new_visible_series = (0..series_count).map(|_| true).collect::<Vec<bool>>();
        map.iter().for_each(|some_idx| {
            if let Some(idx) = some_idx {
                new_visible_series[*idx] = true;
            }
        });
        self.visible_series = new_visible_series;
        let series_count = self.visible_series.iter().filter(|s| **s).count();
        self.offset = Point::new(10.0 * series_count as f64, 0.0);
    }

    /// Draw a series of point to the graph. When drawing the graph, we have to remember that there
    /// is a margin on each side so that no text or line is cut off.
    pub fn draw(
        &self,
        series: &Vec<Vec<Point>>,
        context: &web_sys::CanvasRenderingContext2d,
        mouse: &Option<Point>,
        size: &Size,
        offset: Point,
    ) {
        let off = self.offset + offset;
        let mut selected_points = Vec::new();
        for (idx, s) in series.iter().enumerate() {
            if self.visible_series[idx] {
                if series.len() == 0 {
                    continue;
                }
                context.begin_path();
                context.set_stroke_style(&JsValue::from_str(COLORS[idx % 8]));
                context.move_to(
                    off.x + (s[0].x - self.min.x) * self.scale.x,
                    off.y + (self.max.y - s[0].y) * self.scale.y,
                );
                let mut selected_point = None;
                for point in s.iter() {
                    let x = off.x + (point.x - self.min.x) * self.scale.x;
                    let y = off.y + (self.max.y - point.y) * self.scale.y;
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
                    context.set_fill_style(&JsValue::from_str(COLORS[idx % 8]));
                    context.stroke();
                    context.fill();
                    context
                        .fill_text(
                            &format!("({:.2}, {:.2})", point.x, point.y),
                            size.w + off.x - 3.0,
                            off.y + (idx as f64 * 15.0),
                        )
                        .unwrap();
                }
            });
    }
}
