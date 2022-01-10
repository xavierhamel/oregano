use crate::intrinsics::*;

#[derive(Eq, PartialEq)]
pub enum MouseState {
    Drag,
    Down,
    Up,
    Click,
    None,
}

#[derive(Eq, PartialEq)]
pub enum MouseAction {
    MoveView,
    MoveEntity,
    DrawWire,
    AddComponent,
    None,
}

pub struct Mouse {
    pub screen_pos: Point,
    pub scene_pos: Point,
    pub prev_screen_pos: Point,
    pub state: MouseState,
    pub action: MouseAction,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            prev_screen_pos: Point::new(0.0, 0.0),
            screen_pos: Point::new(0.0, 0.0),
            scene_pos: Point::new(0.0, 0.0),
            state: MouseState::None,
            action: MouseAction::None,
        }    
    }

    pub fn update(&mut self, x: f64, y: f64, offset: Point, scale: f64) {
        self.prev_screen_pos = self.screen_pos;
        self.screen_pos.update(x, y);
        self.scene_pos.update(x / scale + offset.x, y / scale + offset.y);
        self.mousemove(x, y);
    }

    pub fn mousemove(&mut self, x: f64, y: f64) {
        match self.state {
            MouseState::Down => {
                self.state = MouseState::Drag;
            },
            _ => {}
        };
    }

    pub fn mousedown(&mut self, x:f64, y: f64) {
        self.state = MouseState::Down;
    }

    pub fn mouseup(&mut self) {
        if self.state == MouseState::Down {
            self.state = MouseState::Click;
        } else {
            self.state = MouseState::Up;
        }
        self.action = MouseAction::None;
    }
}
