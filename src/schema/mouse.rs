use crate::dom::form::radio;
use crate::intrinsics::*;
use crate::schema::{parts, scene, utils, wires};
use crate::{dom, events};

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
    EditWire,
    ReleaseWire,
    None,
}

impl Action {
    pub fn set_protected(&mut self, action: Self) {
        if self != &Action::DrawWire && self != &Action::EditWire {
            *self = action;
        }
        self.update_ui();
    }

    pub fn set(&mut self, action: Self) {
        *self = action;
        self.update_ui();
    }

    pub fn update_ui(&self) {
        let selected = if *self == Self::DrawWire || *self == Self::EditWire {
            utils::ToolbarTool::Wire
        } else {
            utils::ToolbarTool::Mouse
        };
        let _ = radio::set_checked::<utils::ToolbarTool>("[name=\"toolbar__mouse\"]", selected);

        if let Ok(element) = dom::convert::<web_sys::HtmlElement>(dom::select("body")) {
            if *self == Self::DrawWire {
                let _ = element.style().set_property("cursor", "crosshair");
            } else {
                let _ = element.style().set_property("cursor", "default");
            }
        };
    }
}

pub struct Mouse {
    pub screen_pos: Point,
    pub scene_pos: Point,
    pub prev_screen_pos: Point,
    pub state: State,
    pub action: Action,
    pub ctrl_key: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            prev_screen_pos: Point::new(0.0, 0.0),
            screen_pos: Point::new(0.0, 0.0),
            scene_pos: Point::new(0.0, 0.0),
            state: State::None,
            action: Action::None,
            ctrl_key: false,
        }
    }

    pub fn update(&mut self, event: web_sys::MouseEvent, scene: &scene::Scene) {
        self.ctrl_key = event.ctrl_key();
        let typ = event.type_().parse::<events::Event>();
        let mouse = Point::from(event);
        self.prev_screen_pos = self.screen_pos;
        self.screen_pos = mouse;
        self.scene_pos.update(
            mouse.x / scene.scale + scene.offset.x,
            mouse.y / scene.scale + scene.offset.y,
        );
        match typ {
            Ok(events::Event::MouseMove) => self.mousemove(),
            Ok(events::Event::MouseUp) => self.mouseup(),
            Ok(events::Event::MouseDown) => self.mousedown(),
            _ => {}
        };
    }

    pub fn update_action(&mut self, parts: &parts::Parts, wires: &wires::Wires) {
        let selected_count = parts.selected.len() + wires.selected.len();
        if self.state == State::Down && !self.ctrl_key || self.state == State::Drag {
            self.action.set_protected(if selected_count > 0 {
                Action::MoveEntity
            } else {
                Action::MoveView
            });
        } else if self.state == State::Click {
            self.state = State::Up;
        }
    }

    pub fn mousemove(&mut self) {
        if self.state == State::Down {
            self.state = State::Drag;
            self.action.set_protected(Action::MoveView);
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
            self.action.set_protected(Action::ReleaseEntity);
        } else {
            self.action.set_protected(Action::None);
        }
    }
}
