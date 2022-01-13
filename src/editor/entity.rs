use crate::editor::{component, mouse, property, shape, styles};
use crate::intrinsics::*;
use std::collections::BTreeMap;
use wasm_bindgen::JsValue;
use web_sys;

pub trait Entity: EntityClone {
    fn typ(&self) -> Option<component::components::Components> {
        None
    }

    fn text_position(&self) -> TextPosition {
        TextPosition::Top
    }

    fn rotate_text(&mut self);

    fn properties(&self) -> &BTreeMap<&'static str, property::Property>;
    fn set_properties(&mut self, properties: BTreeMap<&'static str, property::Property>);
    fn set_connection(&mut self, idx: usize, state: bool);
    fn reset_connections(&mut self);
    fn is_wire(&self) -> bool;
    fn size(&self) -> &Size;
    fn set_size(&mut self, size: Size);
    fn origin(&self) -> Point;
    fn set_origin(&mut self, origin: Point);
    fn color(&self) -> Color;
    fn hovered_color(&self) -> Color;
    fn selected_color(&self) -> Color;
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
    fn is_hovered(&self) -> bool;
    fn set_is_hovered(&mut self, is_hovered: bool);

    fn is_draggable(&self) -> bool;

    /// Default implementation to draw an entity.
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        if !self.is_visible() {
            return ();
        }
        context.set_stroke_style(&JsValue::from_str(self.color().0));
        context.set_line_width(1.0);
        let shape = self.shape();
        for shape in &shape.polygones {
            context.begin_path();
            context.move_to(shape[0].x + self.origin().x, shape[0].y + self.origin().y);
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
            context
                .arc(
                    arc.center.x + self.origin().x,
                    arc.center.y + self.origin().y,
                    arc.radius,
                    arc.start / 180.0 * std::f64::consts::PI,
                    arc.end / 180.0 * std::f64::consts::PI,
                )
                .unwrap();
            context.stroke();
        }
        if self.is_selected() {
            let dash_array = js_sys::Array::new_with_length(2);
            dash_array.set(0, JsValue::from_f64(1.5));
            dash_array.set(1, JsValue::from_f64(2.0));
            context.set_line_dash(&dash_array).unwrap();
            self.draw_bounding(
                &context,
                styles::StrokeStyle {
                    line_width: 0.5,
                    color: "#CCC",
                },
            );
            context.set_line_dash(&js_sys::Array::new()).unwrap();
            context.set_stroke_style(&JsValue::from_str(self.selected_color().0));
            //context.set_stroke_style(&JsValue::from_str("#5555FF"));
        }
        if self.is_hovered() || self.is_selected() {
            self.draw_connectors(context);
        }
        context.set_stroke_style(&JsValue::from_str("#FF0000"));
        self.draw_properties(context);
    }

    fn draw_connectors(&self, _context: &web_sys::CanvasRenderingContext2d) {}

    /// Default implementation to draw an entity.
    fn draw_selection(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.draw_bounding(
            context,
            styles::StrokeStyle {
                line_width: 1.0,
                color: "#0000FF",
            },
        );
    }

    /// Draw the bounding box of the entity with it's origin and it's size.
    fn draw_bounding(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        style: styles::StrokeStyle,
    ) {
        context.set_line_width(style.line_width);
        context.set_stroke_style(&JsValue::from_str(style.color));

        context.stroke_rect(
            self.origin().x,
            self.origin().y,
            self.size().w,
            self.size().h,
        );
    }

    fn draw_properties(&self, context: &web_sys::CanvasRenderingContext2d) {
        let mut offset = if self.text_position() == TextPosition::Top {
            context.set_text_align("center");
            context.set_text_baseline("bottom");
            Point::new(self.size().w / 2.0, -3.0)
        } else {
            context.set_text_align("left");
            context.set_text_baseline("middle");
            Point::new(self.size().w + 3.0, self.size().h / 3.0)
        };
        context.set_font("6px Roboto");
        context.set_fill_style(&JsValue::from_str("#FFFFFF"));
        let mut properties = self
            .properties()
            .iter()
            .map(|(&key, property)| (key, property, property::metadata_en(key)))
            .collect::<Vec<(&str, &property::Property, (&str, &str, usize))>>();
        properties.sort_by_key(|k| std::cmp::Reverse(k.2 .2));
        for (_, value, _) in properties.iter() {
            let (text, is_visible) = match value {
                property::Property::Num(_, is_visible) => (value.to_string(), is_visible),
                property::Property::Text(_, is_visible) => (value.to_string(), is_visible),
                property::Property::Unit(_, _, _, is_visible) => (value.to_string(), is_visible),
                _ => ("".to_string(), &false),
            };
            if *is_visible {
                context
                    .fill_text(
                        &text,
                        self.origin().x + offset.x,
                        self.origin().y + offset.y,
                    )
                    .unwrap();
                offset.y -= 8.0;
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
            && point.y <= self.origin().y + self.size().h - margin;
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
        self.rotate_text();
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

#[derive(Clone, PartialEq, Copy)]
pub enum TextPosition {
    Top,
    Left,
}
impl TextPosition {
    pub fn rotate(&mut self) -> Self {
        match self {
            Self::Top => Self::Left,
            Self::Left => Self::Top,
        }
    }
}
