mod layout;
pub mod part;
mod spice;
use crate::schema::{mouse, utils};
use crate::{intrinsics::*, views};
pub use layout::*;
pub use part::*;
pub use spice::*;

pub struct Parts {
    pub parts: Vec<Part>,
    pub hovered: Vec<usize>,
    pub selected: Vec<usize>,
}

impl Parts {
    pub fn new() -> Self {
        //let parts = vec![Part::from(parts::Typ::Voltmeter)];
        Self {
            parts: Vec::new(),
            selected: Vec::new(),
            hovered: Vec::new(),
        }
    }

    pub fn add(&mut self, mut part: Part) {
        if self.floating_part().is_some() {
            self.parts.pop();
        }
        part.state = utils::State::Floating;
        self.parts.push(part);
    }

    pub fn update(&mut self, mouse: &mut mouse::Mouse) {
        let mut selected = Vec::new();
        let mut hovered = Vec::new();
        self.update_selected();
        if mouse.action != mouse::Action::DrawWire {
            self.parts.iter_mut().enumerate().for_each(|(idx, part)| {
                part.mouse_updated(mouse);
                if part.state.is_selected() {
                    selected.push(idx);
                }
                if part.state.is_hovered() {
                    hovered.push(idx);
                }
            });
        }
        self.selected = selected;
        self.hovered = hovered;
        if mouse.state == mouse::State::Down && self.selected.len() == 1 {
            views::properties::update(&self.parts[self.selected[0]]);
        }
        if self.selected.len() != 1 {
            views::properties::empty();
        }
    }

    pub fn update_selected(&mut self) {
        if self.selected.len() == 1 {
            self.parts[self.selected[0]].properties.update_from_inputs();
        }
    }

    pub fn unselect(&mut self) {
        for idx in self.selected.iter() {
            self.parts[*idx].state.set_selected(false);
        }
        self.parts = self
            .parts
            .iter()
            .filter(|part| part.state != utils::State::Floating)
            .map(|part| part.clone())
            .collect::<Vec<Part>>();
        self.selected = Vec::new();
    }

    pub fn delete(&mut self) {
        self.parts = self
            .parts
            .iter()
            .enumerate()
            .filter(|(idx, part)| {
                !self.selected.contains(idx) && part.state != utils::State::Floating
            })
            .map(|(_, part)| part.clone())
            .collect::<Vec<Part>>();
        self.selected = Vec::new();
    }

    pub fn rotate(&mut self) {
        if let Some(floating) = self.floating_part() {
            floating.layout.rotate();
        } else {
            for idx in self.selected.iter() {
                self.parts[*idx].layout.rotate();
            }
        }
    }

    fn floating_part(&mut self) -> Option<&mut Part> {
        if let Some(last) = self.parts.last_mut() {
            if last.state == utils::State::Floating {
                return Some(last);
            }
        }
        None
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Part> {
        self.parts.iter()
    }

    pub fn collide_with_connector(&self, point: Point) -> bool {
        for part in self.parts.iter() {
            if let utils::Colliding::Connector(_) = part.layout.collide_with_point(point) {
                return true;
            }
        }
        false
    }
}
