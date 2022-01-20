use crate::intrinsics::*;
use crate::schema::{parts, scene, wire};
use crate::{clog, events};

#[derive(Eq, PartialEq)]
pub enum State {
    Drag,
    Down,
    Up,
    Click,
    None,
}

#[derive(Eq, PartialEq)]
pub enum Action {
    MoveView,
    MoveEntity,
    ReleaseEntity,
    DrawWire,
    None,
}

impl Action {
    pub fn set(&mut self, action: Self) {
        if self != &Action::DrawWire {
            *self = action;
        }
    }
}

pub struct Mouse {
    pub screen_pos: Point,
    pub scene_pos: Point,
    pub prev_screen_pos: Point,
    pub state: State,
    pub action: Action,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            prev_screen_pos: Point::new(0.0, 0.0),
            screen_pos: Point::new(0.0, 0.0),
            scene_pos: Point::new(0.0, 0.0),
            state: State::None,
            action: Action::None,
        }
    }

    pub fn update(&mut self, mouse: Point, scene: &scene::Scene, event: events::Event) {
        self.prev_screen_pos = self.screen_pos;
        self.screen_pos = mouse;
        self.scene_pos.update(
            mouse.x / scene.scale + scene.offset.x,
            mouse.y / scene.scale + scene.offset.y,
        );
        match event {
            events::Event::MouseMove => self.mousemove(),
            events::Event::MouseUp => self.mouseup(),
            events::Event::MouseDown => self.mousedown(),
            _ => {}
        };
    }

    pub fn update_action(&mut self, parts: &parts::Parts, wires: &wire::Wires) {
        let selected_count = parts.selected.len() + wires.selected.len();
        match self.state {
            State::Down | State::Drag => {
                self.action.set(if selected_count > 0 {
                    Action::MoveEntity
                } else {
                    Action::MoveView
                });
            }
            State::Click => {
                self.state = State::Up;
            }
            _ => {}
        }
    }

    pub fn mousemove(&mut self) {
        if self.state == State::Down {
            self.state = State::Drag;
            self.action.set(Action::MoveView);
        }
    }

    pub fn mousedown(&mut self) {
        self.state = State::Down;
    }

    pub fn mouseup(&mut self) {
        if self.state == State::Down {
            self.state = State::Click;
        } else {
            self.state = State::Up;
        }
        if self.action == Action::MoveEntity {
            self.action.set(Action::ReleaseEntity);
        } else {
            self.action.set(Action::None);
        }
    }
}
