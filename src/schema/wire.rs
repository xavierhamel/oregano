use crate::clog;
use crate::intrinsics::*;
use crate::schema::{ctx, layout, mouse, parts::part};

pub struct Wires {
    pub wires: Vec<Wire>,
    pub selected: Vec<usize>,
}

impl Wires {
    pub fn new() -> Self {
        let wires = vec![Wire::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0))];
        Self {
            wires,
            selected: Vec::new(),
        }
    }

    pub fn add(&mut self, mouse: &mouse::Mouse) {
        self.wires.push(Wire::add(mouse.scene_pos.snap_to_grid()));
    }

    pub fn update(&mut self, mouse: &mut mouse::Mouse) {
        if mouse.action == mouse::Action::DrawWire {
            if let Some(idx) = self.floating_wire() {
                let point = self.wires[idx].layout.shape.points.last().unwrap();
                if mouse.state == mouse::State::Click && self.collide_with_point(*point).len() > 1 {
                    self.end_wire(mouse, false);
                } else {
                    self.wires[idx].trace(mouse);
                }
            } else if mouse.state == mouse::State::Click {
                self.add(mouse);
            }
        } else {
            self.select(mouse);
        }
    }

    pub fn select(&mut self, mouse: &mouse::Mouse) {
        let mut selected = Vec::new();
        self.wires.iter_mut().enumerate().for_each(|(idx, wire)| {
            wire.mouse_updated(mouse);
            if wire.state.is_selected() {
                selected.push(idx);
            }
        });
        self.selected = selected;
    }

    pub fn unselect(&mut self, mouse: &mut mouse::Mouse) {
        self.end_wire(mouse, true);
        for idx in self.selected.iter() {
            self.wires[*idx].state.set_selected(false);
        }
        self.selected = Vec::new();
    }

    pub fn end_wire(&mut self, mouse: &mut mouse::Mouse, do_remove_end: bool) {
        if let Some(idx) = self.floating_wire() {
            if self.wires[idx].layout.shape.points.len() == 2 {
                self.wires.pop();
            } else {
                if do_remove_end {
                    self.wires[idx].layout.trim_shape();
                }
                self.wires[idx].state = part::State::Selected;
            }
            mouse.action = mouse::Action::None;
        }
    }

    fn floating_wire(&self) -> Option<usize> {
        if let Some(last) = self.wires.last() {
            if last.state == part::State::Floating {
                return Some(self.wires.len() - 1);
            }
        }
        None
    }

    pub fn delete(&mut self) {
        self.wires = self
            .wires
            .iter()
            .enumerate()
            .filter(|(idx, _)| !self.selected.contains(idx))
            .map(|(_, wire)| wire.clone())
            .collect::<Vec<Wire>>();
        self.selected = Vec::new();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Wire> {
        self.wires.iter()
    }

    pub fn collide_with_point(&self, point: Point) -> Vec<usize> {
        let mut collisions = Vec::new();
        for (idx, wire) in self.wires.iter().enumerate() {
            if wire.layout.collide_with_point(point) {
                collisions.push(idx);
            }
        }
        collisions
    }
}

#[derive(Clone)]
pub struct Wire {
    layout: layout::WireLayout,
    state: part::State,
    selected_offset: Point,
}

impl Wire {
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            layout: layout::WireLayout::new(start, end),
            state: part::State::None,
            selected_offset: Point::new(0.0, 0.0),
        }
    }

    pub fn add(origin: Point) -> Self {
        Self {
            layout: layout::WireLayout::new(origin, origin),
            state: part::State::Floating,
            selected_offset: Point::new(0.0, 0.0),
        }
    }

    pub fn color() -> &'static str {
        "#575fcf"
    }

    pub fn color_hovered() -> &'static str {
        "#ef5777"
    }

    pub fn color_selected() -> &'static str {
        "#222222"
        //"#ef5777"
    }

    pub fn draw(&self, ctx: &ctx::Ctx) {
        ctx.set_stroke_style(1.0, Self::color());
        self.layout.draw(ctx, &self.state);
        if self.state.is_selected() {
            ctx.set_line_dash_const(vec![2.5, 1.25]);
            ctx.set_stroke_style_const(0.5, "#FFFFFF");
            self.layout.draw(ctx, &self.state);
            ctx.set_line_dash(vec![]);
            self.layout.shape.points.iter().for_each(|&point| {
                ctx.set_fill_style("#ef5777");
                ctx.set_stroke_style_const(1.0, "#FFFFFF");
                ctx.fill_round_rect_const(point, Size::new(7.0, 7.0), 1.3);
                ctx.stroke_round_rect_const(point, Size::new(7.0, 7.0), 1.3);
            })
        }
    }

    pub fn mouse_updated(&mut self, mouse: &mouse::Mouse) {
        let is_hovered = self.layout.collide_with_point(mouse.scene_pos);
        self.state.set_hovered(is_hovered);
        if mouse.state == mouse::State::Down {
            self.state = part::State::None;
            self.state.set_selected(is_hovered);
            if is_hovered {
                self.selected_offset = mouse.scene_pos;
            }
        }
        if (mouse.action == mouse::Action::MoveEntity && self.state.is_selected())
            || self.state == part::State::Floating
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

    pub fn trace(&mut self, mouse: &mouse::Mouse) {
        self.layout.trace(mouse);
    }

    pub fn collide_with_wire(&self, other: &Wire) -> bool {
        let collisions_count_self = self
            .layout
            .extremities()
            .into_iter()
            .filter(|extremity| other.layout.collide_with_point(*extremity))
            .count();
        let collisions_count_other = self
            .layout
            .extremities()
            .into_iter()
            .filter(|extremity| other.layout.collide_with_point(*extremity))
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
                {
                    connectors.push(idx);
                }
            });
        connectors
    }
}
