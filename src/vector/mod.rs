mod overload;
mod conversion;

#[derive(Debug, Clone, Copy)]
pub struct V2<T> {
	pub x: T,
	pub y: T
}

impl<T> V2<T> {
	pub fn new(x: T, y: T) -> Self {
		Self {x, y}
	}
}

#[allow(non_snake_case, unused)]
pub fn V2<T>(x: T, y: T) -> V2<T> { V2::new(x, y) }
#[allow(non_upper_case_globals, unused)]
pub const V2f: fn(f32, f32) -> V2<f32> = |x, y| V2::new(x, y);
#[allow(non_upper_case_globals, unused)]
pub const V2i: fn(i32, i32) -> V2<i32> = |x, y| V2::new(x, y);
#[allow(non_upper_case_globals, unused)]
pub const V2u: fn(u32, u32) -> V2<u32> = |x, y| V2::new(x, y);

macro_rules! impl_number {
	($($type:ident)+) => {
		$(
			impl V2<$type> {
				pub fn null() -> Self {
					Self::new(0 as $type, 0 as $type)
				}

				pub fn is_null(&self) -> bool {
					self.x == 0 as $type && self.y == 0 as $type
				}

				pub fn dot(&self, rhs: Self) -> $type {
					self.x * rhs.x + self.y * rhs.y
				}

				pub fn cros(&self, rhs: Self) -> $type {
					self.x * rhs.y - self.y * rhs.x
				}

				pub fn add(&self, rhs: Self) -> Self {
					*self + rhs
				}

				pub fn sub(&self, rhs: Self) -> Self {
					*self - rhs
				}

				pub fn scale(&self, val: $type) -> Self {
					*self * val
				}

				pub fn unscale(&self, val: $type) -> Self {
					*self / val
				}
			}
		)+
	};
}
impl_number!(i8 i16 i32 i64 u8 u16 u32 u64 f32);

macro_rules! impl_abs {
	($($type:ident)+) => {
		$(
			impl V2<$type> {
				pub fn abs(&self) -> Self {
					Self::new(self.x.abs(), self.y.abs())
				}
			}
		)+
	};
}
impl_abs!(i8 i16 i32 i64 f32);

macro_rules! impl_float {
	($($type:ident)+) => {
		$(
			impl V2<$type> {
				pub fn mag(&self) -> f32 {
					((self.x as f32).powi(2) + (self.y as f32).powi(2)).powf(0.5)
				}

				pub fn dir(&self) -> f32 {
					(self.y as f32).atan2(self.x as f32)
				}

				pub fn set_dir(&self, angle: f32) -> V2<f32> {
					V2::new(angle.cos(), angle.sin()) * self.mag()
				}

				pub fn point_towards(&self, target: Self) -> V2<f32> {
					let diff = target - *self;
					self.set_dir(diff.dir())
				}

				pub fn rotate(&self, angle: f32) -> V2<f32> {
					self.set_dir(self.dir() + angle)
				}

				pub fn rotate_over(&self, origin: V2<f32>, angle: f32) -> V2<f32> {
					let vf32: V2<f32> = Self::into(*self);
					(vf32 - origin).rotate(angle) + origin
				}

				pub fn normalize(&self) -> V2<f32> {
					let vf32: V2<f32> = Self::into(*self);
					vf32 / vf32.mag()
				}

				pub fn distance(&self, rhs: Self) -> f32 {
					(*self - rhs).mag()
				}

				pub fn set(&mut self, rhs: Self) -> &mut Self {
					self.x = rhs.x;
					self.y = rhs.y;
					self
				}

				pub fn add_mut(&mut self, rhs: Self) -> &mut Self {
					*self += rhs;
					self
				}

				pub fn sub_mut(&mut self, rhs: Self) -> &mut Self {
					*self -= rhs;
					self
				}

				pub fn scale_mut(&mut self, val: $type) -> &mut Self {
					*self *= val;
					self
				}

				pub fn unscale_mut(&mut self, val: $type) -> &mut Self {
					*self /= val;
					self
				}
			}
		)+
	};
}
impl_float!(i8 i16 i32 i64 u8 u16 u32 u64 f32);

impl V2<f32> {
	pub fn normalize_mut(&mut self) -> &mut Self {
		if *self == Self::null() {
			self
		}
		else {
			*self /= self.mag();
			self
		}
	}

	pub fn set_dir_mut(&mut self, angle: f32) -> &mut Self {
		let mag = self.mag();
		self.x = angle.cos() * mag;
		self.y = angle.sin() * mag;
		self
	}

	pub fn point_towards_mut(&mut self, target: Self) -> &mut Self {
		self.set_dir_mut((target - *self).dir())
	}

	pub fn rotate_mut(&mut self, angle: f32) -> &mut Self {
		self.set_dir_mut(self.dir() + angle)
	}

	pub fn rotate_over_mut(&mut self, origin: Self, angle: f32) -> &mut Self {
		self.sub_mut(origin).rotate_mut(angle).add_mut(origin)
	}	
}
