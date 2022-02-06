use crate::intrinsics::*;
use crate::schema::{ctx, mouse, parts, properties, utils};
use crate::{error, PARTS};
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Deserialize)]
pub struct Part {
    pub typ: String,
    pub name: Option<String>,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub layout: parts::Layout,
    pub state: utils::State,
    #[serde(default = "Point::default")]
    selected_offset: Point,
    pub properties: properties::Properties,
    colliding: utils::Colliding,
    pub spice: parts::Spice,
}

impl Part {
    pub fn new(
        typ: String,
        layout: parts::Layout,
        properties: properties::Properties,
        spice: parts::Spice,
    ) -> Self {
        let selected_offset = Point::new(layout.size.w / 2.0, layout.size.h / 2.0);
        Self {
            typ,
            name: None,
            layout,
            state: utils::State::None,
            selected_offset,
            properties,
            colliding: utils::Colliding::None,
            spice,
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
            ctx.set_line_dash_const(vec![]);
        }
        if self.state != utils::State::None {
            self.layout.draw_connectors(&self.state, ctx);
        }
    }

    pub fn collide_with_point(&mut self, point: Point) -> &utils::Colliding {
        self.colliding = self.layout.collide_with_point(point);
        &self.colliding
    }

    pub fn mouse_updated(&mut self, mouse: &mut mouse::Mouse) {
        self.collide_with_point(mouse.scene_pos);
        let is_hovered = self.colliding == utils::Colliding::Shape;
        self.state.set_hovered(is_hovered);
        if mouse.state == mouse::State::Down {
            if self.state == utils::State::Floating {
                self.layout.snap_to_grid();
            }
            self.state = utils::State::None;
            self.state.set_selected(is_hovered);
            if is_hovered {
                self.selected_offset = mouse.scene_pos - self.layout.origin;
            }
        }
        if (mouse.action == mouse::Action::MoveEntity && self.state.is_selected())
            || self.state == utils::State::Floating
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
                _ => Err(Box::new(error::Sim::MissingConnectionPart(0))),
            })
            .collect::<Result<Vec<String>, Box<error::Sim>>>()
    }

    pub fn to_spice(&self) -> Result<String, error::Error> {
        self.spice.to_spice(&self.properties, self.connectors()?)
    }
}

impl std::str::FromStr for Part {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Part, error::Error> {
        let data = s.split(",").collect::<Vec<&str>>();
        let mut part = PARTS.get(data[0])?;
        part.layout.update_from_str(data[1])?;
        let properties = data[2].parse::<properties::Properties>()?;
        part.properties = properties;
        Ok(part)
    }
}

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
