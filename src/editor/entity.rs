use std::collections::hash_map;
use wasm_bindgen::JsValue;
use web_sys;
use crate::editor::{styles, shape, mouse, component};
use crate::intrinsics::*;
use crate::{unit, dom};
use crate::log;

#[derive(PartialEq, Clone)]
pub enum Property {
    Text(String, bool),
    Num(f64, bool),
    Unit(f64, unit::Prefix, unit::Unit, bool),
    InternalStr(&'static str),
    InternalF64(f64),
}

impl Property {
    pub fn to_string(&self) -> String {
        match self {
            Self::Text(value, _) => value.clone(),
            Self::Num(value, _) => value.to_string(),
            Self::Unit(value, prefix, unit, _) => format!("{} {}{}", value, prefix.to_string(), unit.to_string()),
            Self::InternalStr(value) => value.to_string(),
            Self::InternalF64(value) => value.to_string(),
        }
    }

    pub fn from_inputs(prefix: &str) -> Result<Self, ()> {
        let is_visible = match dom::form::checkbox::value_as_bool(&format!("[name=\"{}-is-visible\"]", prefix)) {
            Ok(value) => value,
            Err(_) => { return Err(()) },
        };
        let value = match dom::form::text_input::value_as_string(&format!("[name=\"{}\"]", prefix)) {
            Ok(val) => val,
            Err(_) => { return Err(()) },
        };
        let unit_value = dom::form::text_input::value_as_string(&format!("[name=\"{}-unit\"]", prefix));
        let prefix_value = dom::form::select::value_as_usize(&format!("[name=\"{}-unit-prefix\"]", prefix));
        match (unit_value, prefix_value) {
            (Ok(unit_str), Ok(prefix_idx)) => {
                if let Ok(f64_value) = value.parse::<f64>() {
                    let unit = unit::Unit::from_str(&unit_str);
                    Ok(Self::Unit(f64_value, unit::Prefix::as_array()[prefix_idx].clone(), unit, is_visible))
                } else {
                    Err(())
                }
            },
            _ => {
                if let Ok(f64_value) = value.parse::<f64>() {
                    Ok(Self::Num(f64_value, is_visible))
                } else {
                    Ok(Self::Text(value, is_visible))
                }
            }
        }
    }
}

pub trait Entity: EntityClone {
    fn typ(&self) -> Option<component::components::Components> {
        None
    }

    fn properties(&self) -> &hash_map::HashMap<&'static str, Property>;
    fn set_properties(&mut self, properties: hash_map::HashMap<&'static str, Property>);
    fn properties_keys(&self) -> hash_map::Keys<&'static str, Property>;
    fn set_connection(&mut self, idx: usize, state: bool);
    fn reset_connections(&mut self);
    fn is_wire(&self) -> bool;
    fn size(&self) -> &Size;
    fn set_size(&mut self, size: Size);
    fn origin(&self) -> Point;
    fn set_origin(&mut self, origin: Point);
    fn color(&self) -> &Color;
    fn is_visible(&self) -> bool;
    fn set_is_visible(&mut self, is_visible: bool);

    fn connections(&self) -> &Vec<Point>;
    fn set_connections(&mut self, connections: Vec<Point>);

    /// Return all the necessery element to draw the component.
    fn shape(&self) -> &shape::Shape;
    fn shape_mut(&mut self) -> &mut shape::Shape;
    fn set_shape(&mut self, shape: shape::Shape);

    /// Selected offset is the offset between the origin of the entity and the position of the
    /// mouse when the entity is selected or moved around.
    fn selected_offset(&self) -> Point;
    fn set_selected_offset(&mut self, mouse_position: Point);
    fn is_selected(&self) -> bool;
    fn set_is_selected(&mut self, is_selected: bool);

    fn is_draggable(&self) -> bool;

    /// Default implementation to draw an entity.
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        if !self.is_visible() {
            return ();
        }
        if self.is_selected() {
            context.set_stroke_style(&JsValue::from_str("#ef5777"));
            //context.set_stroke_style(&JsValue::from_str("#5555FF"));
        } else {
            context.set_stroke_style(&JsValue::from_str(self.color().0));
        }
        context.set_line_width(1.0);
        let shape = self.shape();
        for shape in &shape.polygones {
            context.begin_path();
            context.move_to(
                shape[0].x + self.origin().x,
                shape[0].y + self.origin().y,
            );
            for idx in 1..shape.len() {
                context.line_to(
                    shape[idx].x + self.origin().x,
                    shape[idx].y + self.origin().y,
                );
            }
            context.stroke();
        }
        for arc in &shape.arcs {
            context.begin_path();
            context.arc(
                arc.center.x + self.origin().x,
                arc.center.y + self.origin().y,
                arc.radius,
                arc.start / 180.0 * std::f64::consts::PI,
                arc.end / 180.0 * std::f64::consts::PI,
            ).unwrap();
            context.stroke();
        }
        context.set_stroke_style(&JsValue::from_str("#FF0000"));
        self.draw_connectors(context);
        self.draw_properties(context);
    }

    fn draw_connectors(&self, context: &web_sys::CanvasRenderingContext2d) {}

    /// Default implementation to draw an entity.
    fn draw_selection(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.draw_bounding(
            context,
            styles::StrokeStyle {
                line_width:1.0,
                color: "#0000FF"
            }
        );
    }

    /// Draw the bounding box of the entity with it's origin and it's size.
    fn draw_bounding(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        style: styles::StrokeStyle
    ) {
        context.set_line_width(style.line_width);
        context.set_stroke_style(&JsValue::from_str(style.color));

        context.stroke_rect(
            self.origin().x,
            self.origin().y,
            self.size().w,
            self.size().h
        );
    }

    fn draw_properties(&self, context: &web_sys::CanvasRenderingContext2d) {
        let mut offset = 5.0;
        context.set_font("8px sans-serif");
        context.set_fill_style(&JsValue::from_str("#FFFFFF"));
        for (_, value) in self.properties().iter() {
            let (text, is_visible) = match value {
                Property::Num(_, is_visible) => (value.to_string(), is_visible),
                Property::Text(_, is_visible) => (value.to_string(), is_visible),
                Property::Unit(_, _, _, is_visible) => (value.to_string(), is_visible),
                Property::InternalStr(_) => ("".to_string(), &false),
                Property::InternalF64(_) => ("".to_string(), &false),
            };
            if *is_visible {
                context.fill_text(&text, self.origin().x + 5.0, self.origin().y + offset).unwrap();
                offset -= 10.0;
            }
        }
    }

    /// Return if the entity is colliding with a given point. The offset is the offset of the
    /// scene. The hit box is a little smaller than the actual size because of when we draw
    /// wires to be able to connect the wires without selecting the component.
    fn collide_with_point(&self, point: Point) -> bool {
        let margin = 0.0;
        return point.x >= self.origin().x + margin
            && point.x <= self.origin().x + self.size().w - margin
            && point.y >= self.origin().y + margin
            && point.y <= self.origin().y + self.size().h - margin
    }
    
    fn connections_collide_with_point(&self, point: Point) -> Option<usize> {
        let mut idx = 0;
        for connection in self.connections() {
            let c = *connection + self.origin();
            if c.snap_to_grid() == point.snap_to_grid() {
                return Some(idx);
            }
            idx += 1;
        }
        None
    }

    /// Rotate the entity by a 90 degrees.
    fn rotate(&mut self) {
        let old_center = Point::new(
            self.origin().x + self.size().w / 2.0,
            self.origin().y + self.size().h / 2.0,
        );
        self.shape_mut().rotate();
        let translation = self.shape().bounding_box().0;
        self.shape_mut().translate(translation);
        self.update_bounding_box();
        let new_center = Point::new(
            self.origin().x + self.size().w / 2.0,
            self.origin().y + self.size().h / 2.0,
        );
        let mut connections = shape::Shape::new(vec![self.connections().clone()], vec![]);
        connections.rotate();
        connections.translate(translation);
        connections.snap_to_grid_polys();
        log(&format!("x1: {}, y1: {}, x2: {}, y2: {}", connections.polygones[0][0].x, connections.polygones[0][0].y, connections.polygones[0][1].x, connections.polygones[0][1].y));
        self.set_connections(connections.polygones[0].clone());
        self.set_origin(self.origin() + (old_center - new_center));
    }

    fn update_bounding_box(&mut self) {
        let (min, max) = self.shape().bounding_box();
        self.set_size(Size::new(max.x - min.x, max.y - min.y));
    }

    fn drag(&mut self, mouse: &mouse::Mouse) {
        if self.is_draggable() {
            self.set_origin(mouse.scene_pos - self.selected_offset());
        }
    }

    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait EntityClone {
    fn clone_box(&self) -> Box<dyn Entity>;
}

impl<T> EntityClone for T
where
    T: 'static + Entity + Clone,
{
    fn clone_box(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Entity> {
    fn clone(&self) -> Box<dyn Entity> {
        self.clone_box()
    }
}

