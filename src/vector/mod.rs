mod chain_op;
mod overload_op;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    pub fn mag(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).powf(0.5)
    }

    pub fn dir(&self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl Into<Vec2> for (f64, f64) {
    fn into(self) -> Vec2 {
        Vec2 {x: self.0, y: self.1}
    }
}

impl Into<(f64, f64)> for Vec2 {
    fn into(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl Into<(u16, u16)> for Vec2 {
    fn into(self) -> (u16, u16) {
        (self.x as u16, self.y as u16)
    }
}

impl Into<(u16, u16)> for &mut Vec2 {
    fn into(self) -> (u16, u16) {
        (self.x as u16, self.y as u16)
    }
}