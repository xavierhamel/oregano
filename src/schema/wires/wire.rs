use crate::schema::{ctx, mouse, parts::part, utils, wires};
use crate::{error, intrinsics::*};
use std::fmt;

#[derive(Clone)]
pub struct Wire {
    pub layout: wires::Layout,
    pub state: utils::State,
    selected_offset: Point,
    selected_corner: Option<usize>,
    colliding: utils::Colliding,
}

impl Wire {
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            layout: wires::Layout::new(start, end),
            state: utils::State::None,
            selected_offset: Point::new(0.0, 0.0),
            selected_corner: None,
            colliding: utils::Colliding::None,
        }
    }

    pub fn add(origin: Point) -> Self {
        Self {
            layout: wires::Layout::new(origin, origin),
            state: utils::State::Floating,
            selected_offset: Point::new(0.0, 0.0),
            selected_corner: None,
            colliding: utils::Colliding::None,
        }
    }

    pub fn color() -> &'static str {
        "#575fcf"
    }

    pub fn _color_hovered() -> &'static str {
        "#ef5777"
    }

    pub fn _color_selected() -> &'static str {
        "#222222"
    }

    pub fn draw(&self, ctx: &ctx::Ctx) {
        ctx.set_stroke_style(1.0, Self::color());
        self.layout.draw(ctx, &self.state);
        if self.state.is_selected() {
            self.layout.shape.points.iter().for_each(|&point| {
                ctx.set_fill_style("#ef5777");
                ctx.set_stroke_style_const(1.0, "#000000");
                ctx.fill_round_rect_const(point, Size::new(7.0, 7.0), 1.3);
                ctx.stroke_round_rect_const(point, Size::new(7.0, 7.0), 1.3);
            })
        }
    }

    pub fn collide_with_point(&mut self, point: Point) -> &utils::Colliding {
        self.colliding = self.layout.collide_with_point(point);
        &self.colliding
    }

    pub fn mouse_updated(&mut self, mouse: &mut mouse::Mouse, keep_selected: bool) {
        if let Some(corner) = self.selected_corner {
            if mouse.state == mouse::State::Up {
                self.selected_corner = None;
                mouse.action.set(mouse::Action::ReleaseWire);
                self.layout.end_edit_shape();
            } else {
                self.layout.edit_shape(mouse, corner);
            }
            return;
        }
        let is_hovered = self.collide_with_point(mouse.scene_pos) != &utils::Colliding::None;
        self.state.set_hovered(is_hovered);
        if mouse.state == mouse::State::Down {
            if self.state.is_selected() {
                self.try_select_corner(mouse);
            }
            if ((mouse.ctrl_key && !self.state.is_selected()) || !mouse.ctrl_key) && !keep_selected
            {
                self.state = utils::State::None;
                self.state.set_selected(is_hovered);
            }
            if is_hovered || (keep_selected && mouse.action != mouse::Action::MoveEntity) {
                self.selected_offset = mouse.scene_pos;
            }
        }
        if (mouse.action == mouse::Action::MoveEntity && self.state.is_selected())
            || self.state == utils::State::Floating
        {
            self.layout
                .shape
                .translate(mouse.scene_pos - self.selected_offset);
            self.selected_offset = mouse.scene_pos;
        }
        if mouse.action == mouse::Action::ReleaseEntity {
            self.layout.shape.snap_to_grid();
        }
    }

    fn try_select_corner(&mut self, mouse: &mut mouse::Mouse) {
        if self.selected_corner.is_none() {
            if let Some(corner) = self.layout.collide_with_corners(mouse.scene_pos) {
                self.selected_corner = Some(corner);
                mouse.action.set(mouse::Action::EditWire);
            }
        }
    }

    pub fn trace(&mut self, mouse: &mouse::Mouse) {
        self.layout.trace(mouse);
    }

    pub fn collide_with_wire(&self, other: &Wire) -> bool {
        let collisions_count_self = self
            .layout
            .extremities()
            .into_iter()
            .filter(|extremity| {
                other.layout.collide_with_point(*extremity) != utils::Colliding::None
            })
            .count();
        let collisions_count_other = self
            .layout
            .extremities()
            .into_iter()
            .filter(|extremity| {
                other.layout.collide_with_point(*extremity) != utils::Colliding::None
            })
            .count();
        collisions_count_self > 0 || collisions_count_other > 0
    }

    pub fn collide_with_part(&self, part: &part::Part) -> Vec<usize> {
        let mut connectors = Vec::new();
        part.layout
            .connectors
            .iter()
            .enumerate()
            .for_each(|(idx, connector)| {
                if self
                    .layout
                    .collide_with_point(connector.origin + part.layout.origin)
                    != utils::Colliding::None
                {
                    connectors.push(idx);
                }
            });
        connectors
    }
}

impl std::iter::FromIterator<Point> for Wire {
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Self {
        let mut wire = Wire::new(Point::new(0.0, 0.0), Point::new(0.0, 0.0));
        wire.layout.shape.points = iter.into_iter().collect::<Vec<Point>>();
        wire
    }
}

impl std::str::FromStr for Wire {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Wire, error::Error> {
        s.split(";")
            .filter(|s| s.len() > 0)
            .map(|point| Point::from_str(point))
            .collect::<Result<Wire, error::Error>>()
    }
}

impl fmt::Debug for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self
            .layout
            .shape
            .points
            .iter()
            .fold(String::new(), |mut acc, point| {
                acc.push_str(&format!("{};", point));
                acc
            });
        write!(f, "{}", out)
    }
}
