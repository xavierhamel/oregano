use std::collections::hash_map;
use wasm_bindgen::JsValue;
pub mod components;
pub mod lumped;
pub mod source;
pub mod probe;
pub mod dialogs;

use crate::intrinsics::*;
use crate::editor::{shape, entity, wire};


/// Return the color that should be a component
pub fn color() -> Color {
    Color("#10ac84")
    //Color("#ef5777")
}

/// A component in an electrical circuit. This does not define how the component is simulated, it
/// only define how a component is drawn to the editor and how it can connect with wires. To define
/// how a component opperates in a simulation, go in the src/simulation folder. This struct only
/// degine the visual aspect of a component and how to represent it in a graph.
///
/// All components in the editor (except for wires) are represented by this struct. The only
/// difference is the value stored in those fields. To generate a specific component, use the enum
/// `editor::component::Components` or generate a custom component using the
/// `editor::component::Component::new()` function.
#[derive(Clone)]
pub struct Component {
    pub typ: components::Components,
    short_name: &'static str,
    origin: Point,
    color: Color,
    size: Size,
    is_selected: bool,
    selected_offset: Point,
    shape: shape::Shape,
    connections: Vec<Point>,
    connected_connections: Vec<bool>,
    pub connected_to: Vec<(usize, String)>,
    is_visible: bool,
    pub properties: hash_map::HashMap<&'static str, entity::Property>
}

impl Component {
    pub fn new(
        typ: components::Components,
        short_name: &'static str,
        origin: Point,
        size: Size,
        color: Color,
        shape: shape::Shape,
        connections: Vec<Point>,
        properties: hash_map::HashMap<&'static str, entity::Property>,
    ) -> Self {
        let connected_connections = (0..connections.len())
            .map(|_| false)
            .collect::<Vec<bool>>();
        Self {
            typ,
            short_name,
            origin,
            size,
            color,
            is_selected: false,
            selected_offset: Point::new(0.0, 0.0),
            shape,
            connections,
            connected_connections,
            is_visible: true,
            connected_to: Vec::new(),
            properties,
        }
    }
}

impl entity::Entity for Component {
    fn typ(&self) -> Option<components::Components> {
        Some(self.typ)
    }

    fn properties(&self) -> &hash_map::HashMap<&'static str, entity::Property> {
        &self.properties
    }

    fn set_properties(&mut self, properties: hash_map::HashMap<&'static str, entity::Property>) {
        self.properties = properties;
    }

    fn properties_keys(&self) -> hash_map::Keys<&'static str, entity::Property> {
        self.properties.keys()
    }
    
    fn reset_connections(&mut self) {
        self.connected_connections = (0..self.connections.len())
            .map(|_| false)
            .collect::<Vec<bool>>();
    }

    fn set_connection(&mut self, idx: usize, state: bool) {
        self.connected_connections[idx] = state;
    }

    fn is_wire(&self) -> bool {
        false
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
        // We must be sure that connections point are on the grid. We can take an assumption that
        // if one of the connection point is on the grid, all connections points are on the grid.
        let conn = self.connections()[0] + origin;
        let diff = conn.snap_to_grid() - conn;
        self.origin = origin + diff;
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn draw_connectors(&self, context: &web_sys::CanvasRenderingContext2d) {
        for (idx, connection) in self.connections().iter().enumerate() {
            context.begin_path();
            context.arc(
                connection.x + self.origin().x,
                connection.y + self.origin().y,
                2.0,
                0.0,
                std::f64::consts::PI * 2.0,
            ).unwrap();
            if self.connected_connections[idx] {
                context.set_fill_style(&JsValue::from_str(wire::color().0));
                context.set_stroke_style(&JsValue::from_str(wire::color().0));
                context.fill();
            } else {
                context.set_stroke_style(&JsValue::from_str("#FF0000"));
            }
            context.stroke();
        }
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
