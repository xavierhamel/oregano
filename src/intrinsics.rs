use crate::error;
use serde::Deserialize;
use std::fmt;

/// Represent a coordinate in the canvas. When there's an x and a y coord, use this to represent
/// it.
#[derive(Clone, Copy, PartialEq, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const GRID_SIZE: f64 = 10.0;

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn update(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn distance(&self, other: Point) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }

    pub fn snap_to_grid(&self) -> Self {
        Self {
            x: (self.x / Self::GRID_SIZE).round() * Self::GRID_SIZE,
            y: (self.y / Self::GRID_SIZE).round() * Self::GRID_SIZE,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<web_sys::MouseEvent> for Point {
    fn from(event: web_sys::MouseEvent) -> Self {
        Self {
            x: event.offset_x() as f64,
            y: event.offset_y() as f64,
        }
    }
}

impl std::str::FromStr for Point {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Point, error::Error> {
        let data = s.split(",").collect::<Vec<&str>>();
        if data.len() != 2 {
            Err(Box::new(error::Import::MissingToken))
        } else {
            match (data[0].parse::<f64>(), data[1].parse::<f64>()) {
                (Ok(x), Ok(y)) => Ok(Point::new(x, y)),
                _ => Err(Box::new(error::Import::UnexpectedValue)),
            }
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x:{},y:{})", self.x, self.y)
    }
}

/// Represent the size of something in the canvas. It is the same thing as the point but the
/// difference is the name of the field of the struct.
#[derive(Clone, Copy, PartialEq, Deserialize)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

impl Size {
    pub fn new(w: f64, h: f64) -> Self {
        Self { w, h }
    }

    pub fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl std::ops::Add for Size {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            w: self.w + other.w,
            h: self.h + other.h,
        }
    }
}

impl std::ops::Sub for Size {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            w: self.w - other.w,
            h: self.h - other.h,
        }
    }
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(w:{},h:{})", self.w, self.h)
    }
}

#[derive(Clone, PartialEq)]
pub struct Color(pub &'static str);
