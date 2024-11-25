use crate::Vec2;

impl Vec2 {
    pub fn normalize(&mut self) -> &mut Self {
        let mag = self.mag();
        if mag != 0.0 {
            self.scale(1.0/mag);
        }
        self
    }

    pub fn add(&mut self, rhs: Vec2) -> &mut Self {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }

    pub fn sub(&mut self, rhs: Vec2) -> &mut Self {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }

    pub fn scale(&mut self, value: f64) -> &mut Self {
        self.x *= value;
        self.y *= value;
        self
    }

    pub fn set_direction(&mut self, rad: f64) -> &mut Self {
        self.x = rad.cos() * self.mag();
        self.y = rad.sin() * self.mag();
        self
    }

    pub fn rotate(&mut self, rad: f64) -> &mut Self {
        self.set_direction(rad + self.dir());
        self
    }

    pub fn rotate_over(&mut self, origin: Vec2, rad: f64) -> &mut Self {
        self.sub(origin);
        self.rotate(rad);
        self.add(origin);
        self
    }

    pub fn point_towards(&mut self, mut target: Vec2) -> &mut Self {
        self.set_direction(target.sub(*self).dir())
    }
}