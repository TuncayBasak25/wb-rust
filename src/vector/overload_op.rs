use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::Vec2;

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Add<(f64, f64)> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: (f64, f64)) -> Vec2 {
        Vec2 {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Sub<(f64, f64)> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: (f64, f64)) -> Vec2 {
        Vec2 {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl AddAssign<(f64, f64)> for Vec2 {
    fn add_assign(&mut self, rhs: (f64, f64)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl SubAssign<(f64, f64)> for Vec2 {
    fn sub_assign(&mut self, rhs: (f64, f64)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}