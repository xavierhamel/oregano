use std::collections::hash_map;
use wasm_bindgen::JsValue;
use crate::intrinsics::*;
use crate::editor::{shape, entity, entity::Entity, styles};
use crate::log;

/// Return the color that should be a component
pub fn color() -> Color {
    Color("#575fcf")
    //Color("#ef5777")
}

/// Represent a wire in the schema. It is use to connect multiple components.
#[derive(Clone, PartialEq)]
pub struct Wire {
    origin: Point,
    size: Size,
    color: Color,
    is_selected: bool,
    selected_offset: Point,
    pub shape: shape::Shape,
    connections: Vec<Point>,
    is_visible: bool,
    pub is_visited: bool,
    properties: hash_map::HashMap<&'static str, entity::Property>,
}

impl Wire {
    pub fn new(
        start: Point,
        end: Point,
        color: Color,
    ) -> Self {
        let mut wire = Self {
            origin: start,
            size: Size::new(0.0, 0.0),
            color,
            is_selected: false,
            selected_offset: Point::new(0.0, 0.0),
            shape: shape::Shape::new(vec![], vec![]),
            connections: vec![],
            is_visible: true,
            is_visited: false,
            properties: hash_map::HashMap::new(),
        };
        wire.update_shape(start, end);
        wire.update_size();
        wire
    }
    
    pub fn update_size(&mut self) {
        self.size = Size::new(self.shape.polygones[0][1].x, self.shape.polygones[0][1].y);
    }

    pub fn update_shape(&mut self, start: Point, end: Point) {
        let mut end_point = Point::new(end.x - self.origin.x, 0.0);
        if (start.x - end.x).abs() < (start.y - end.y).abs() {
            end_point = Point::new(0.0, end.y - self.origin.y);
        }

        self.shape = shape::Shape::new(
            vec![vec![Point::new(0.0, 0.0), end_point.snap_to_grid()]],
            vec![]
        );
        self.origin = start.snap_to_grid();
    }

    fn draw_wire(&self, context: &web_sys::CanvasRenderingContext2d, style: styles::StrokeStyle) {
        context.set_line_width(style.line_width);
        context.set_stroke_style(&JsValue::from_str(style.color));
        let points = &self.shape().polygones[0]
            .iter()
            .map(|&point| {
                point + self.origin
            })
            .collect::<Vec<Point>>();
        context.begin_path();
        context.move_to(points[0].x, points[0].y);
        context.line_to(points[1].x, points[1].y);
        //context.line_to(points[2].x, points[2].y);
        context.stroke();
    }
}

impl entity::Entity for Wire {
    fn properties(&self) -> &hash_map::HashMap<&'static str, entity::Property> {
        &self.properties
    }

    fn set_properties(&mut self, properties: hash_map::HashMap<&'static str, entity::Property>) {
        self.properties = properties;
    }

    fn properties_keys(&self) -> hash_map::Keys<&'static str, entity::Property> {
        self.properties.keys()
    }

    fn set_connection(&mut self, idx: usize, state: bool) {}

    fn reset_connections(&mut self) {}

    fn is_wire(&self) -> bool {
        true
    }

    fn connections(&self) -> &Vec<Point> {
        &self.connections
    }

    fn set_connections(&mut self, connections: Vec<Point>) {
        self.connections = connections
    }

    fn size(&self) -> &Size {
        &self.size
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn origin(&self) -> Point {
        self.origin
    }

    fn set_origin(&mut self, origin: Point) {
        self.origin = origin.snap_to_grid();
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn set_selected_offset(&mut self, mouse_position: Point) {
        self.selected_offset = mouse_position;
    }

    fn selected_offset(&self) -> Point {
        self.selected_offset
    }

    fn is_selected(&self) -> bool {
        self.is_selected
    }

    fn set_is_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }

    fn is_draggable(&self) -> bool {
        true
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_is_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }

    fn shape_mut(&mut self) -> &mut shape::Shape {
        &mut self.shape
    }

    fn shape(&self) -> &shape::Shape {
        &self.shape
    }

    fn set_shape(&mut self, shape: shape::Shape) {
        self.shape = shape;
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        if self.is_selected {
            self.draw_selection(context);
        } else {
            self.draw_wire(
                context,
                styles::StrokeStyle {
                    line_width:1.0,
                    color:self.color.0,
                }
            )
        }
    }

    fn draw_selection(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.draw_wire(
            context,
            styles::StrokeStyle {
                line_width:1.0,
                color:"#ef5777",
            }
        )
    }

    /// Return if the wire is colliding with a given point. The offset is the offset of the
    /// scene. The hit box is a little larger than the actual size because of the wire is 1px thick
    /// so very hard to hit.
    fn collide_with_point(&self, point: Point) -> bool {
        let start = self.shape.polygones[0][0] + self.origin;
        let end = self.shape.polygones[0][1] + self.origin;
        let margin = 4.0;
        // horizontal
        if end.x != 0.0 && point.y >= start.y - margin && point.y <= start.y + margin {
            if end.x > start.x {
                return point.x >= start.x && point.x <= end.x;
            }
            return point.x >= end.x && point.x <= start.x;
        } else if end.y != 0.0 && point.x >= start.x - margin && point.x <= start.x + margin {
            if end.y > start.y {
                return point.y >= start.y && point.y <= end.y;
            }
            return point.y >= end.y && point.y <= start.y;
        }
        return false;
    }

    /// Wires cannot be rotated (for now)
    fn rotate(&mut self) {

    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
