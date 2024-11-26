<<<<<<< HEAD
mod check;
mod chain_op;
mod angle;
=======
mod conversion;
mod float;
mod overload_op;
>>>>>>> aefcecf7af3e859521706e4ebaeccde73557ddd6

use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};

#[derive(Clone, Copy, Debug, Default)]
<<<<<<< HEAD
pub struct V2(f32, f32);

impl V2 {
    pub fn x(&self) -> f32{
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn mag(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).powf(0.5)
    }

    pub fn dir(&self) -> V2 {
        let mut unit = self.clone();
        unit.normalize();
        unit
    }
=======
pub struct Vec2<T>
{
	pub x: T,
    pub y: T
}

impl<T> Vec2<T>
where T: Copy + 
        Add<Output = T> + Sub<Output = T> + Mul<Output = T> +
        AddAssign + SubAssign
{
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }

	pub fn add<U: Into<Vec2<T>>>(&mut self, rhs: U) -> &mut Self {
		let rhs = rhs.into();
		self.x += rhs.x;
		self.y += rhs.y;
		self
	}
	
	pub fn sub<U: Into<Vec2<T>>>(&mut self, rhs: U) -> &mut Self {
		let rhs = rhs.into();
		self.x -= rhs.x;
		self.y -= rhs.y;
		self
	}
	
	pub fn dot<U: Into<Vec2<T>>>(&self, other: U) -> T {
		let other = other.into();
		self.x * other.x + self.y * other.y
	}
	
	pub fn cross<U: Into<Vec2<T>>>(&self, other: U) -> T {
		let other = other.into();
		self.x * other.y - self.y * other.x
	}
>>>>>>> aefcecf7af3e859521706e4ebaeccde73557ddd6
}