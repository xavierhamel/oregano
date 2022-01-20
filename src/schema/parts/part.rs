use crate::error;
use crate::intrinsics::*;
use crate::schema::{ctx, layout, mouse, parts, properties};
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum State {
    Floating,
    Hovered,
    Selected,
    SelectedAndHovered,
    None,
}

impl State {
    pub fn set_selected(&mut self, is_selected: bool) {
        *self = if is_selected {
            match self {
                State::None | State::Selected | State::Floating => State::Selected,
                State::Hovered | State::SelectedAndHovered => State::SelectedAndHovered,
            }
        } else {
            match self {
                State::Selected | State::None => State::None,
                State::SelectedAndHovered | State::Hovered => State::Hovered,
                State::Floating => State::Floating,
            }
        }
    }

    pub fn set_hovered(&mut self, is_hovered: bool) {
        *self = if is_hovered {
            match self {
                State::Selected | State::SelectedAndHovered => State::SelectedAndHovered,
                State::None | State::Hovered => State::Hovered,
                State::Floating => State::Floating,
            }
        } else {
            match self {
                State::Selected | State::SelectedAndHovered => State::Selected,
                State::None | State::Hovered => State::None,
                State::Floating => State::Floating,
            }
        }
    }

    pub fn is_hovered(&self) -> bool {
        *self == State::Hovered || *self == State::SelectedAndHovered
    }

    pub fn is_selected(&self) -> bool {
        *self == State::Selected || *self == State::SelectedAndHovered
    }
}

#[derive(PartialEq, Clone)]
pub enum Colliding {
    Connector(usize),
    Shape,
    None,
}

#[derive(Clone)]
pub struct Part {
    pub typ: parts::Typ,
    pub layout: layout::PartLayout,
    pub state: State,
    selected_offset: Point,
    pub properties: properties::Properties,
    colliding: Colliding,
}

impl Part {
    pub fn new(
        typ: parts::Typ,
        layout: layout::PartLayout,
        properties: properties::Properties,
    ) -> Self {
        let selected_offset = Point::new(layout.size.w / 2.0, layout.size.h / 2.0);
        Self {
            typ,
            layout,
            state: State::None,
            selected_offset,
            properties,
            colliding: Colliding::None,
        }
    }

    pub fn color() -> &'static str {
        "#10ac84"
    }

    pub fn selected_color() -> &'static str {
        "#ef5777"
    }

    pub fn draw(&self, ctx: &ctx::Ctx) {
        ctx.set_stroke_style(1.0, Self::color());
        self.layout.draw(ctx);
        if self.state.is_selected() {
            ctx.set_line_dash_const(vec![2.5, 1.25]);
            ctx.set_stroke_style_const(0.5, "#CCCCCC");
            self.layout.draw_bounding(ctx);
            ctx.set_line_dash(vec![]);
        }
        if self.state != State::None {
            self.layout.draw_connectors(&self.state, ctx);
        }
    }

    pub fn collide_with_point(&mut self, point: Point) -> &Colliding {
        self.colliding = self.layout.collide_with_point(point);
        &self.colliding
    }

    pub fn mouse_updated(&mut self, mouse: &mouse::Mouse) {
        let is_hovered = self.collide_with_point(mouse.scene_pos) == &Colliding::Shape;
        self.state.set_hovered(is_hovered);
        if mouse.state == mouse::State::Down {
            self.state = State::None;
            self.state.set_selected(is_hovered);
            if is_hovered {
                self.selected_offset = mouse.scene_pos - self.layout.origin;
            }
        }
        if (mouse.action == mouse::Action::MoveEntity && self.state.is_selected())
            || self.state == State::Floating
        {
            self.layout.origin = mouse.scene_pos - self.selected_offset;
        } else if mouse.action == mouse::Action::ReleaseEntity {
            self.layout.snap_to_grid();
        }
    }

    pub fn connectors(&self) -> Result<Vec<String>, Box<error::Sim>> {
        self.layout
            .connectors
            .iter()
            .map(|conn| match &conn.connected_to {
                Some(name) => Ok(name.clone()),
                _ => Err(Box::new(error::Sim::MissingConnection(0, 0))),
            })
            .collect::<Result<Vec<String>, Box<error::Sim>>>()
    }
}

impl From<parts::Typ> for Part {
    fn from(typ: parts::Typ) -> Self {
        match typ {
            parts::Typ::Voltmeter => parts::probe::voltmeter("P0".to_string()),
            parts::Typ::SourceVoltageAc => parts::source::voltage_ac("Vac".to_string()),
            parts::Typ::SourceVoltageDc => parts::source::voltage_dc("Vdc".to_string()),
            parts::Typ::SourceCurrentAc => parts::source::current_ac("Iac".to_string()),
            parts::Typ::SourceCurrentDc => parts::source::current_dc("Idc".to_string()),
            parts::Typ::Resistor => parts::lumped::resistor("R0".to_string()),
            parts::Typ::Capacitor => parts::lumped::capacitor("C0".to_string()),
            parts::Typ::Inductor => parts::lumped::inductor("L0".to_string()),
            parts::Typ::Ground => parts::lumped::ground("0".to_string()),
            parts::Typ::Node => parts::lumped::node("n0".to_string()),
        }
    }
}

// impl std::str::FromStr for Part {
//     fn from_str(s: &str)
// }

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.typ)
    }
}

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{:?},{:?}", self.typ, self.layout, self.properties)
    }
}
