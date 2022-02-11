mod layout;
pub mod wire;
use crate::intrinsics::*;
use crate::schema::{mouse, parts, utils};
pub use layout::*;
pub use wire::*;

pub struct Wires {
    pub wires: Vec<Wire>,
    pub selected: Vec<usize>,
    copied: Vec<usize>,
}

impl Wires {
    pub fn new() -> Self {
        let wires = vec![Wire::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0))];
        Self {
            wires,
            selected: Vec::new(),
            copied: Vec::new(),
        }
    }

    pub fn add(&mut self, mouse: &mouse::Mouse) {
        self.wires.push(Wire::add(mouse.scene_pos.snap_to_grid()));
    }

    pub fn update(&mut self, mouse: &mut mouse::Mouse, parts: &parts::Parts, keep_selected: bool) {
        if mouse.action == mouse::Action::DrawWire {
            if let Some(idx) = self.floating_wire() {
                let point = self.wires[idx].layout.shape.points.last().unwrap();
                if mouse.state == mouse::State::Click
                    && (self.collide_with_point(*point).len() > 1
                        || parts.collide_with_connector(*point))
                {
                    self.end_wire(mouse, false);
                } else {
                    self.wires[idx].trace(mouse);
                }
            } else if mouse.state == mouse::State::Click {
                self.add(mouse);
            }
        } else {
            self.select(mouse, keep_selected);
        }
    }

    pub fn select(&mut self, mouse: &mut mouse::Mouse, keep_selected: bool) {
        let mut selected = Vec::new();
        self.wires.iter_mut().enumerate().for_each(|(idx, wire)| {
            wire.mouse_updated(mouse, keep_selected);
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
            if self.wires[idx].layout.shape.points.len() == 2 && do_remove_end {
                self.wires.pop();
            } else {
                if do_remove_end {
                    self.wires[idx].layout.trim_shape();
                }
                self.wires[idx].state = utils::State::Selected;
            }
            mouse.action.set(mouse::Action::None);
        }
    }

    fn floating_wire(&self) -> Option<usize> {
        if let Some(last) = self.wires.last() {
            if last.state == utils::State::Floating {
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

    pub fn copy(&mut self) {
        self.copied = self.selected.clone();
    }

    pub fn paste(&mut self) {
        let mut selected = Vec::new();
        for idx in &self.copied {
            selected.push(self.wires.len());
            let mut wire = self.wires[*idx].clone();
            wire.layout.shape.translate(Point::new(10.0, 10.0));
            self.wires.push(wire);
            self.wires[*idx].state = utils::State::None;
        }
        self.selected = selected;
    }

    pub fn do_keep_selected(&mut self, mouse: &mouse::Mouse) -> bool {
        for &idx in self.selected.iter() {
            if self.wires[idx].collide_with_point(mouse.scene_pos) == &utils::Colliding::Shape {
                return true;
            }
        }
        return false;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Wire> {
        self.wires.iter()
    }

    pub fn collide_with_point(&self, point: Point) -> Vec<usize> {
        let mut collisions = Vec::new();
        for (idx, wire) in self.wires.iter().enumerate() {
            if wire.layout.collide_with_point(point) != utils::Colliding::None {
                collisions.push(idx);
            }
        }
        collisions
    }
}
