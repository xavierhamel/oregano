#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y:f64) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn update(&mut self, x:f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn distance(&self, other: Point) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }

    pub fn snap_to_grid(&self) -> Self {
        let grid_size = 5.0;
        Self {
            x: (self.x / grid_size).round() * grid_size,
            y: (self.y / grid_size).round() * grid_size,
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

#[derive(Clone, Copy, PartialEq)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

impl Size {
    pub fn new (w: f64, h:f64) -> Self {
        Self {
            w,
            h
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Color(pub &'static str);
