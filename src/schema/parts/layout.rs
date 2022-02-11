use crate::intrinsics::*;
use crate::schema::{ctx, error, layout::*, parts, shape, utils};
use crate::sim::circuit;
use crate::LAYOUTS;
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Deserialize)]
pub struct Layout {
    pub origin: Point,
    pub base: Option<String>,
    pub size: Size,
    #[serde(default = "Orientation::default")]
    orientation: Orientation,
    pub shape: shape::Shape,
    pub connectors: Vec<Connector>,
}

impl Layout {
    pub fn to_json(&self) -> String {
        let mut conns = self.connectors.iter().fold(String::new(), |mut acc, c| {
            acc.push_str(&format!(
                "        {{\"origin\":{{\"x\":{},\"y\":{}}}}},\n",
                c.origin.x, c.origin.y
            ));
            acc
        });
        conns.pop();
        conns.pop();
        format!(
            "{{
    \"origin\":{{\"x\":{}, \"y\":{}}},
    \"size\":{{\"w\":{},\"h\":{}}},
    \"shape\":{},
    \"connectors\":[\n{}\n    ]
}}",
            self.origin.x,
            self.origin.y,
            self.size.w,
            self.size.h,
            self.shape.to_json(),
            conns,
        )
    }

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
            base: None,
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

    pub fn translate(&mut self, offset: Point) {
        self.origin = self.origin + offset;
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

    pub fn draw_connectors(&self, state: &utils::State, ctx: &ctx::Ctx) {
        self.connectors
            .iter()
            .for_each(|connector| connector.draw(self.origin, state, ctx));
    }

    pub fn collide_with_point(&self, point: Point) -> utils::Colliding {
        if let Some(idx) = self.connectors_collide_with_point(point) {
            utils::Colliding::Connector(idx)
        } else if self.shape.collide_with_point(self.origin, point) {
            utils::Colliding::Shape
        } else {
            utils::Colliding::None
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

    pub fn update_from_str(&mut self, s: &str) -> Result<(), error::Error> {
        let data = s.split("!").collect::<Vec<&str>>();
        if data.len() != 3 {
            return Err(Box::new(error::Import::MissingToken));
        } else {
            let origin = match (data[0].parse::<f64>(), data[1].parse::<f64>()) {
                (Ok(x), Ok(y)) => Point::new(x, y),
                _ => return Err(Box::new(error::Import::UnexpectedValue)),
            };
            let orientation = data[2].parse::<Orientation>()?;
            for _ in 0..orientation.rotation_count() {
                self.rotate();
            }
            self.origin = origin;
        }
        Ok(())
    }
}

impl fmt::Debug for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}!{}!{:?}",
            self.origin.x, self.origin.y, self.orientation
        )
    }
}

impl std::str::FromStr for Layout {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, error::Error> {
        LAYOUTS.get(s)
    }
}
