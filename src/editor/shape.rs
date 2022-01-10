use crate::intrinsics::*;

#[derive(Clone, PartialEq)]
pub struct Shape {
    pub polygones: Vec<Vec<Point>>,
    pub arcs: Vec<Arc>,
}

impl Shape {
    pub fn new(polygones: Vec<Vec<Point>>, arcs: Vec<Arc>) -> Self {
        Self {
            polygones,
            arcs,
        }
    }

    /// Rotate the shape by a 90 degrees.
    pub fn rotate(&mut self) {
        let size = self.size();
        let mut polygones = Vec::new();
        let mut arcs = Vec::new();
        for poly in self.polygones.iter() {
            let mut points = Vec::new();
            for point in poly.iter() {
                let x = -point.y;
                let y = point.x;
                points.push(Point::new(x, y));
            }
            polygones.push(points);
        }
        for arc in self.arcs.iter() {
            let x = -arc.center.y;
            let y = arc.center.x;
            let start = arc.start + 90.0;
            let end = arc.end + 90.0;
            arcs.push(
                Arc::new(Point::new(x, y), arc.radius, start, end)
            );
        }
        self.polygones = polygones;
        self.arcs = arcs;
    }

    // Translate all the points of a shape be
    pub fn translate(&mut self, translation: Point) {
        let mut polygones = Vec::new();
        let mut arcs = Vec::new();
        for poly in self.polygones.iter() {
            let mut points = Vec::new();
            for point in poly.iter() {
                points.push(*point - translation);
            }
            polygones.push(points);
        }
        for arc in self.arcs.iter() {
            let mut new_arc = arc.clone();
            new_arc.center = new_arc.center - translation;
            arcs.push(new_arc);
        }
        self.polygones = polygones;
        self.arcs = arcs;
    }

    /// Return the bounding box of the shape relative to the offset where the first point is the
    /// top left corner and the second point is the bottom right corner of the bounding box.
    pub fn bounding_box(&self) -> (Point, Point) {
        let mut min = self.polygones[0][0];
        let mut max = self.polygones[0][0];
        for shape in self.polygones.iter() {
            for point in shape.iter() {
                if point.x < min.x {
                    min.x = point.x;
                }
                if point.y < min.y {
                    min.y = point.y;
                }
                if point.x > max.x {
                    max.x = point.x;
                }
                if point.y > max.y {
                    max.y = point.y;
                }
            }
        }
        for arc in self.arcs.iter() {
            if arc.center.x - arc.radius < min.x {
                min.x = arc.center.x - arc.radius;
            }
            if arc.center.x + arc.radius > max.x {
                max.x = arc.center.x + arc.radius;
            }
            if arc.center.y - arc.radius < min.y {
                min.y = arc.center.y - arc.radius;
            }
            if arc.center.y + arc.radius > max.y {
                max.y = arc.center.y + arc.radius;
            }
        }
        (min, max)
    }

    pub fn size(&self) -> Size {
        let (min, max) = self.bounding_box();
        Size::new(max.x - min.x, max.y - min.y)
    }

    pub fn snap_to_grid_polys(&mut self) {
        for poly in &mut self.polygones {
            for point in poly {
                *point = point.snap_to_grid();
            }
        }

    }
}

#[derive(Clone, PartialEq)]
pub struct Arc {
    pub center: Point,
    pub radius: f64,
    pub start: f64,
    pub end: f64,
}

impl Arc {
    pub fn new(center: Point, radius: f64, start: f64, end: f64) -> Self {
        Self {
            center,
            radius,
            start,
            end
        }
    }
}
