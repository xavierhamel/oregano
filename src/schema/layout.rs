use crate::error;
use crate::intrinsics::*;
use crate::schema::{ctx, parts, shape, utils};
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Deserialize)]
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

    pub fn rotation_count(&self) -> usize {
        match self {
            Orientation::North => 0,
            Orientation::East => 1,
            Orientation::South => 2,
            Orientation::West => 3,
        }
    }

    pub fn default() -> Self {
        Self::North
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

#[derive(Clone, Deserialize)]
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

    pub fn draw(&self, origin: Point, state: &utils::State, ctx: &ctx::Ctx) {
        if state.is_selected() {
            self.draw_circle(origin, ctx);
        } else if state == &utils::State::Hovered && self.connected_to.is_none() {
            self.draw_cross(origin, ctx);
        }
    }

    fn draw_circle(&self, origin: Point, ctx: &ctx::Ctx) {
        let circle = shape::Arc::circle(Point::new(0.0, 0.0), 1.5);
        if self.connected_to.is_some() {
            ctx.set_stroke_style(0.5, "#CCC");
            ctx.set_fill_style(parts::Part::selected_color());
            ctx.fill_arc(origin + self.origin, &circle);
        } else {
            ctx.set_stroke_style(0.5, parts::Part::selected_color());
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
