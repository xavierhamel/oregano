use crate::intrinsics::*;
use crate::schema::ctx;

/// A poly represent a polygones (but it is more a line that can be close). When drawn, it will
/// link each point one after the other. To close the shape, the last point must be the same as the
/// first one.
#[derive(Clone, PartialEq)]
pub struct Poly {
    pub points: Vec<Point>,
}

impl Poly {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    /// Rotate by 90 deg all the points where the origin of the rotation is (0., 0.)
    pub fn rotate(&mut self) {
        self.points.iter_mut().for_each(|point| {
            point.update(-point.y, point.x);
        });
    }

    /// Translate all the points by a specified value
    pub fn translate(&mut self, offset: Point) {
        self.points.iter_mut().for_each(|point| {
            *point = *point + offset;
        });
    }

    /// The minimum and maximum value in x and y.
    pub fn bounding(&self) -> (Point, Point) {
        let mut min = self.points[0];
        let mut max = self.points[0];
        self.points.iter().for_each(|point| {
            min.x = point.x.min(min.x);
            min.y = point.y.min(min.y);
            max.x = point.x.max(max.x);
            max.y = point.y.max(max.y);
        });
        (min, max)
    }

    /// Will move the cursor to the first point and draw a line between each points. It will not
    /// acutally stroke the path, you will need to call ctx.stroke() for that.
    pub fn add_path(&self, origin: Point, ctx: &ctx::Ctx) {
        if self.points.len() == 0 {
            return;
        }
        ctx.move_to(self.points[0] + origin);
        self.points
            .iter()
            .for_each(|point| ctx.line_to(origin + *point));
    }

    /// Find the shortest distance between a point and one of the segment of the polygone
    pub fn shortest_distance_with_point(&self, origin: Point, point: Point) -> Option<f64> {
        if self.points.len() == 0 {
            return None;
        }
        let mut min_distance = (origin + self.points[0]).distance(point);
        for idx in 1..self.points.len() {
            min_distance = min_distance.min(self.shortest_distance_point_and_segment(
                point,
                self.points[idx - 1] + origin,
                self.points[idx] + origin,
            ));
        }
        Some(min_distance)
    }

    fn shortest_distance_point_and_segment(&self, point: Point, start: Point, end: Point) -> f64 {
        let length = start.distance(end);
        let dot_product =
            (point.x - start.x) * (end.x - start.x) + (point.y - start.y) * (end.y - start.y);
        let projection = dot_product / length;
        if projection < 0.0 {
            point.distance(start)
        } else if projection > length {
            point.distance(end)
        } else {
            let cross_product = ((point.x - start.x) * (end.y - start.y)
                - (point.y - start.y) * (end.x - start.x))
                .abs();
            cross_product / length
        }
    }

    pub fn snap_to_grid(&mut self) {
        if self.points.len() > 0 {
            let diff = self.points[0].snap_to_grid() - self.points[0];
            self.translate(diff);
        }
    }
}

/// An arc is a circle or a part of a circle.
#[derive(Clone)]
pub struct Arc {
    center: Point,
    radius: f64,
    start: f64,
    end: f64,
}

impl Arc {
    pub fn new(center: Point, radius: f64, start: f64, end: f64) -> Self {
        Self {
            center,
            radius,
            start,
            end,
        }
    }

    pub fn circle(center: Point, radius: f64) -> Self {
        Self {
            center,
            radius,
            start: 0.0,
            end: 360.0,
        }
    }

    /// Rotate by 90 deg all the points where the origin of the rotation is (0., 0.)
    pub fn rotate(&mut self) {
        self.center.update(-self.center.y, self.center.x);
        self.start = self.start + 90.0;
        self.end = self.end + 90.0;
    }

    /// Translate all the points by a specified value
    pub fn translate(&mut self, offset: Point) {
        self.center = self.center + offset;
    }

    /// The minimum and maximum value in x and y.
    pub fn bounding(&self) -> (Point, Point) {
        let min = self.center - Point::new(self.radius, self.radius);
        let max = self.center + Point::new(self.radius, self.radius);
        (min, max)
    }

    // Add an arc to the current path
    pub fn add_path(&self, origin: Point, ctx: &ctx::Ctx) {
        let angle = self.start / 180.0 * std::f64::consts::PI;
        let offset = Point::new(angle.cos() * self.radius, angle.sin() * self.radius);
        ctx.move_to(self.center + origin + offset);
        ctx.arc(self.center + origin, self.radius, self.start, self.end);
    }
}

/// A shape is a group of arcs and polygones that are drawn togheter.
#[derive(Clone)]
pub struct Shape {
    polys: Vec<Poly>,
    arcs: Vec<Arc>,
    pub size: Size,
}

impl Shape {
    pub fn new(polys: Vec<Vec<Point>>, arcs: Vec<Arc>) -> Self {
        let ps = polys
            .into_iter()
            .map(|p| Poly::new(p))
            .collect::<Vec<Poly>>();
        let mut shape = Self {
            polys: ps,
            arcs,
            size: Size::new(0.0, 0.0),
        };
        shape.size();
        shape
    }
    /// Rotate by 90 deg all the points where the origin of the rotation is (0., 0.)
    pub fn rotate(&mut self) {
        self.polys.iter_mut().for_each(|poly| {
            poly.rotate();
        });
        self.arcs.iter_mut().for_each(|arc| {
            arc.rotate();
        });
    }

    /// Translate all the points by a specified value
    pub fn translate(&mut self, offset: Point) {
        self.polys.iter_mut().for_each(|poly| {
            poly.translate(offset);
        });
        self.arcs.iter_mut().for_each(|arc| {
            arc.translate(offset);
        });
    }

    /// The minimum and maximum value in x and y.
    pub fn bounding(&self) -> (Point, Point) {
        let mut min = Point::new(f64::MAX, f64::MAX);
        let mut max = Point::new(f64::MIN, f64::MIN);
        self.polys
            .iter()
            .map(|poly| poly.bounding())
            .chain(self.arcs.iter().map(|arc| arc.bounding()))
            .for_each(|(s_min, s_max)| {
                min.x = s_min.x.min(min.x);
                min.y = s_min.y.min(min.y);
                max.x = s_max.x.max(max.x);
                max.y = s_max.y.max(max.y);
            });
        (min, max)
    }

    /// The size of the shape
    pub fn size(&mut self) -> Size {
        let (min, max) = self.bounding();
        self.size = Size::new(max.x - min.x, max.y - min.y);
        self.size
    }

    /// Add the path of the shape to the ctx
    pub fn add_path(&self, origin: Point, ctx: &ctx::Ctx) {
        self.polys
            .iter()
            .for_each(|poly| poly.add_path(origin, ctx));
        self.arcs.iter().for_each(|arc| arc.add_path(origin, ctx));
    }

    /// Return if the shape is colliding with a given point. The collisions are computed as if the
    /// shape was a rectangle and not the actual shape.
    pub fn collide_with_point(&self, offset: Point, point: Point) -> bool {
        return point.x >= offset.x
            && point.x <= offset.x + self.size.w
            && point.y >= offset.y
            && point.y <= offset.y + self.size.h;
    }
}
