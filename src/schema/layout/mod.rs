pub mod shape;
use crate::error;
use crate::intrinsics::*;
use crate::schema::parts::part;
use crate::schema::{ctx, mouse, wire};
use crate::sim::circuit;
use std::fmt;

#[derive(Clone)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn rotate_clockwise(&mut self) {
        *self = match *self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        };
    }
}

impl fmt::Debug for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Orientation::North => "n",
            Orientation::East => "e",
            Orientation::South => "s",
            Orientation::West => "w",
        };
        write!(f, "{}", out)
    }
}

impl std::str::FromStr for Orientation {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Orientation::North),
            "e" => Ok(Orientation::East),
            "s" => Ok(Orientation::South),
            "w" => Ok(Orientation::West),
            _ => Err(Box::new(error::Import::UnexpectedValue)),
        }
    }
}

#[derive(Clone)]
pub struct Connector {
    pub origin: Point,
    pub connected_to: Option<String>,
}

impl Connector {
    const RADIUS: f64 = 2.0;

    pub fn new(origin: Point) -> Self {
        Self {
            origin,
            connected_to: None,
        }
    }

    pub fn draw(&self, origin: Point, state: &part::State, ctx: &ctx::Ctx) {
        if state.is_selected() {
            self.draw_circle(origin, ctx);
        } else if state == &part::State::Hovered && self.connected_to.is_none() {
            self.draw_cross(origin, ctx);
        }
    }

    fn draw_circle(&self, origin: Point, ctx: &ctx::Ctx) {
        let circle = shape::Arc::circle(Point::new(0.0, 0.0), 1.5);
        if self.connected_to.is_some() {
            ctx.set_stroke_style(0.5, "#CCC");
            ctx.set_fill_style(part::Part::selected_color());
            ctx.fill_arc(origin + self.origin, &circle);
        } else {
            ctx.set_stroke_style(0.5, part::Part::selected_color());
        }
        ctx.stroke_arc(origin + self.origin, &circle);
    }

    fn draw_cross(&self, origin: Point, ctx: &ctx::Ctx) {
        ctx.set_stroke_style_const(0.5, "#AAA");
        ctx.stroke_shape(
            origin + self.origin,
            &shape::Shape::new(
                vec![
                    vec![Point::new(-5.0, 0.0), Point::new(5.0, 0.0)],
                    vec![Point::new(0.0, -5.0), Point::new(0.0, 5.0)],
                ],
                vec![],
            ),
        );
    }

    pub fn collide_with_point(&self, offset: Point, point: Point) -> bool {
        (offset + self.origin).distance(point) <= Connector::RADIUS
    }

    pub fn rotate(&mut self) {
        self.origin.update(-self.origin.y, self.origin.x);
    }

    pub fn translate(&mut self, offset: Point) {
        self.origin = self.origin + offset;
    }
}

#[derive(Clone)]
pub struct PartLayout {
    pub origin: Point,
    pub size: Size,
    orientation: Orientation,
    shape: shape::Shape,
    pub connectors: Vec<Connector>,
}

impl PartLayout {
    pub fn new(origin: Point, shape: shape::Shape, connectors: Vec<Point>) -> Self {
        let cs = connectors
            .iter()
            .map(|c| Connector::new(*c))
            .collect::<Vec<Connector>>();
        Self {
            origin,
            size: shape.size,
            orientation: Orientation::North,
            shape,
            connectors: cs,
        }
    }

    /// This will rotate everything by 90 degrees.
    pub fn rotate(&mut self) {
        let old_center = self.center();
        self.shape.rotate();
        let offset = -self.shape.bounding().0;
        self.shape.translate(offset);
        self.size = self.shape.size();
        if self.connectors.len() > 0 {
            let anchor = self.connectors[0].origin;
            self.connectors.iter_mut().for_each(|connector| {
                connector.rotate();
                connector.translate(offset);
            });
            self.origin = self.origin - self.connectors[0].origin + anchor;
        } else {
            self.origin = self.origin + (old_center - self.center());
        }

        self.orientation.rotate_clockwise();
    }

    /// The center of the shape (based on it's size)
    pub fn center(&self) -> Point {
        Point::new(self.size.w / 2.0, self.size.h / 2.0) + self.origin
    }

    pub fn _mirror(&mut self) {}

    pub fn draw(&self, ctx: &ctx::Ctx) {
        ctx.stroke_shape(self.origin, &self.shape);
    }

    pub fn draw_bounding(&self, ctx: &ctx::Ctx) {
        ctx.stroke_rect(self.origin, self.shape.size);
    }

    pub fn draw_connectors(&self, state: &part::State, ctx: &ctx::Ctx) {
        self.connectors
            .iter()
            .for_each(|connector| connector.draw(self.origin, state, ctx));
    }

    pub fn collide_with_point(&self, point: Point) -> part::Colliding {
        if let Some(idx) = self.connectors_collide_with_point(point) {
            part::Colliding::Connector(idx)
        } else if self.shape.collide_with_point(self.origin, point) {
            part::Colliding::Shape
        } else {
            part::Colliding::None
        }
    }

    fn connectors_collide_with_point(&self, point: Point) -> Option<usize> {
        for (idx, c) in self.connectors.iter().enumerate() {
            if c.collide_with_point(self.origin, point) {
                return Some(idx);
            }
        }
        None
    }

    pub fn snap_to_grid(&mut self) {
        let conn = self.connectors[0].origin + self.origin;
        let diff = conn.snap_to_grid() - conn;
        self.origin = self.origin + diff;
    }

    pub fn connect(&mut self, connection: &circuit::Connection, name: &str) {
        self.connectors[connection.connector].connected_to = Some(name.to_string());
    }
}

impl fmt::Debug for PartLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{:?}",
            self.origin.x, self.origin.y, self.orientation
        )
    }
}

#[derive(Clone)]
pub struct WireLayout {
    pub shape: shape::Poly,
}

impl WireLayout {
    pub fn new(start: Point, end: Point) -> Self {
        let shape = shape::Poly::new(vec![start, end]);
        Self { shape }
    }

    pub fn draw(&self, ctx: &ctx::Ctx, state: &part::State) {
        let len = self.shape.points.len();
        if state == &part::State::Floating && len > 1 {
            ctx.stroke_points(&self.shape.points[..len - 1]);
            ctx.set_stroke_style_const(0.5, wire::Wire::color());
            ctx.set_line_dash(vec![2.0, 1.0]);
            ctx.stroke_points(&self.shape.points[len - 2..]);
            ctx.set_line_dash(vec![]);
        } else {
            ctx.stroke_poly(Point::new(0.0, 0.0), &self.shape);
        }
    }

    pub fn collide_with_point(&self, point: Point) -> bool {
        if let Some(distance) = self
            .shape
            .shortest_distance_with_point(Point::new(0.0, 0.0), point)
        {
            distance < 3.0
        } else {
            false
        }
    }

    pub fn trace(&mut self, mouse: &mouse::Mouse) {
        if mouse.state == mouse::State::Click {
            self.shape.points.push(mouse.scene_pos.snap_to_grid());
        }
        let len = self.shape.points.len();
        let diff = self.shape.points[len - 2] - mouse.scene_pos;
        if diff.x.abs() > diff.y.abs() {
            self.shape.points[len - 1] =
                Point::new(mouse.scene_pos.x, self.shape.points[len - 2].y).snap_to_grid();
        } else {
            self.shape.points[len - 1] =
                Point::new(self.shape.points[len - 2].x, mouse.scene_pos.y).snap_to_grid();
        }
    }

    pub fn trim_shape(&mut self) {
        self.shape.points.pop();
    }

    pub fn extremities(&self) -> Vec<Point> {
        match self.shape.points.last() {
            Some(last) => vec![self.shape.points[0], *last],
            _ => vec![self.shape.points[0]],
        }
    }
}

// impl fmt::Debug for WireLayout {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{},{}", self.origin.x, self.origin.y)
//     }
// }
