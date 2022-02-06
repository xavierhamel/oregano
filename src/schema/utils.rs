use crate::error;
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Colliding {
    Connector(usize),
    Shape,
    None,
}

impl<'de> Deserialize<'de> for Colliding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Colliding::None)
    }
}

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

impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(State::None)
    }
}

pub enum ToolbarTool {
    Mouse,
    Wire,
}

impl std::str::FromStr for ToolbarTool {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, error::Error> {
        match s {
            "mouse" => Ok(Self::Mouse),
            "wire" => Ok(Self::Wire),
            _ => Err(Box::new(error::Internal::Parse)),
        }
    }
}

impl fmt::Display for ToolbarTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wire => write!(f, "wire"),
            Self::Mouse => write!(f, "mouse"),
        }
    }
}
