use std::{f32::consts::PI, ops::{AddAssign, Deref, DerefMut}};

use super::V2;

pub struct Rad(f32);
pub struct Deg(f32);

macro_rules! impl_angle {
    ($t:ty) => {
        impl $t {
            pub fn value(&self) -> f32 {
                self.0
            }

            pub fn add<T: Into<$t>>(&mut self, rhs: T) -> &mut Self {
                let rhs: $t = rhs.into();
                self.0 += rhs.value();
                self
            }

            pub fn cos(&self) -> f32 {
                $t(self.value()).into().value().cos()
            }

            pub fn sin(&self) -> f32 {
                $t(self.value()).into().value().sin()
            }
        }
    };
}

impl From<Deg> for Rad {
    fn from(value: Deg) -> Self {
        Self(value.0 / 180.0 * PI)
    }
}

impl From<Rad> for Deg {
    fn from(value: Rad) -> Self {
        Self(value.0 / PI * 180.0)
    }
}

impl From<V2> for Rad {
    fn from(v2: V2) -> Self {
        assert!(v2.is_null(), "Null vector does not have a direction!");
        Self(v2.y().atan2(v2.x()))
    }
}

impl From<V2> for Deg {
    fn from(v2: V2) -> Self {
        assert!(v2.is_null(), "Null vector does not have a direction!");
        Self(v2.y().atan2(v2.x()) / PI * 180.0)
    }
}

impl From<Rad> for V2 {
    fn from(angle: Rad) -> Self {
        Self(angle.value().cos(), angle.value().sin())
    }
}

impl From<Deg> for V2 {
    fn from(angle: Deg) -> Self {
        let angle: Rad = angle.into();
        Self(angle.value().cos(), angle.value().sin())
    }
}

impl_angle!(Rad);
impl_angle!(Deg);
