use super::{angle::{Angle, Rad}, V2};

impl V2 {
    pub fn add(&mut self, rhs: V2) -> &mut Self {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self
    }

    pub fn sub(&mut self, rhs: V2) -> &mut Self {
        self.0 -= rhs.x();
        self.1 -= rhs.y();
        self
    }

    pub fn scale(&mut self, value: f32) -> &mut Self {
        self.0 *= value;
        self.1 *= value;
        self
    }

    pub fn normalize(&mut self) -> &mut Self {
        let mag = self.mag();
        if mag != 0.0 {
            self.scale(1.0/mag);
        }
        self
    }

    pub fn set_direction<U: Into<Rad>>(&mut self, angle: U) -> &mut Self {
        if !self.is_null() {
            let mag = self.mag();
            let mut angle: Rad = angle.into();
            self.0 = angle.cos() * mag;
            self.1 = angle.sin() * mag;
        }
        self
    }

    pub fn rotate(&mut self, rad: f32) -> &mut Self {
        if !self.is_null() {
            self.set_direction(rad + self.dir().unwrap());
        }
        self
    }

    pub fn rotate_over(&mut self, origin: V2, rad: f64) -> &mut Self {
        self.sub(origin);
        self.rotate(rad);
        self.add(origin);
        self
    }

    pub fn point_towards(&mut self, mut target: Vec2) -> &mut Self {
        self.set_direction(target.sub(*self).dir())
    }
}