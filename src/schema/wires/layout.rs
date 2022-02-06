use crate::intrinsics::*;
use crate::schema::{ctx, mouse, shape, utils, wires};

#[derive(Clone)]
pub struct Layout {
    pub shape: shape::Poly,
}

impl Layout {
    pub fn new(start: Point, end: Point) -> Self {
        let shape = shape::Poly::new(vec![start, end]);
        Self { shape }
    }

    pub fn draw(&self, ctx: &ctx::Ctx, state: &utils::State) {
        let len = self.shape.points.len();
        if state == &utils::State::Floating && len > 1 {
            ctx.stroke_points(&self.shape.points[..len - 1]);
            ctx.set_stroke_style_const(0.5, wires::Wire::color());
            ctx.set_line_dash(vec![2.0, 1.0]);
            ctx.stroke_points(&self.shape.points[len - 2..]);
            ctx.set_line_dash(vec![]);
        } else {
            ctx.stroke_poly(Point::new(0.0, 0.0), &self.shape);
        }
    }

    pub fn collide_with_corners(&self, point: Point) -> Option<usize> {
        self.shape
            .points
            .iter()
            .enumerate()
            .fold(None, |acc, (idx, corner)| {
                if corner.distance(point) <= 5.0 {
                    Some(idx)
                } else {
                    acc
                }
            })
    }

    pub fn collide_with_point(&self, point: Point) -> utils::Colliding {
        if let Some(idx) = self.collide_with_corners(point) {
            return utils::Colliding::Connector(idx);
        } else if let Some(distance) = self.shape.distance_with_point(Point::new(0.0, 0.0), point) {
            if distance < 4.0 {
                return utils::Colliding::Shape;
            }
        }
        utils::Colliding::None
    }

    pub fn trace(&mut self, mouse: &mouse::Mouse) {
        if mouse.state == mouse::State::Down {
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

    pub fn edit_shape(&mut self, mouse: &mouse::Mouse, idx: usize) {
        self.shape.points[idx] = mouse.scene_pos;
    }

    pub fn end_edit_shape(&mut self) {
        self.shape.snap_to_grid();
    }
}
