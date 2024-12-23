use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::V2;

macro_rules! impl_neg {
	($($type:ident)+) => {
		$(
			impl Neg for V2<$type> {
				type Output = Self;
				fn neg(self) -> Self::Output {
					Self::new(-self.x, -self.y)
				}
			}
		)+
	};
}
impl_neg!(i8 i16 i32 i64 f32);

macro_rules! impl_arith {
	($($type:ident)+) => {
		$(
			impl Add for V2<$type> {
				type Output = Self;
				fn add(self, rhs: Self) -> Self::Output {
					Self::new(self.x + rhs.x, self.y + rhs.y)
				}
			}

			impl Sub for V2<$type> {
				type Output = Self;
				fn sub(self, rhs: Self) -> Self::Output {
					Self::new(self.x - rhs.x, self.y - rhs.y)
				}
			}

			impl Mul<$type> for V2<$type> {
				type Output = Self;
				fn mul(self, rhs: $type) -> Self::Output {
					Self::new(self.x * rhs, self.y * rhs)
				}
			}

			impl Mul<V2<$type>> for $type {
				type Output = V2<$type>;
				fn mul(self, rhs: V2<$type>) -> Self::Output {
					Self::Output::new(self * rhs.x, self * rhs.y)
				}
			}

			impl AddAssign for V2<$type> {
				fn add_assign(&mut self, rhs: Self) {
					self.x += rhs.x;
					self.y += rhs.y;
				}
			}

			impl SubAssign for V2<$type> {
				fn sub_assign(&mut self, rhs: Self) {
					self.x -= rhs.x;
					self.y -= rhs.y;
				}
			}

			impl MulAssign<$type> for V2<$type> {
				fn mul_assign(&mut self, rhs: $type) {
					self.x *= rhs;
					self.y *= rhs;
				}
			}
		)+
	};
}
impl_arith!(i8 i16 i32 i64 u8 u16 u32 u64 f32);

macro_rules! impl_div {
	($($type:ident)+) => {$(
		impl Div<$type> for V2<$type> {
			type Output = Self;
			fn div(self, rhs: $type) -> Self::Output {
				assert!(rhs != 0 as $type, "Division by zero!");
				Self::new(self.x / rhs, self.y / rhs)
			}
		}

		impl DivAssign<$type> for V2<$type> {
			fn div_assign(&mut self, rhs: $type) {
				assert!(rhs != 0 as $type, "Division by zero!");
				self.x /= rhs;
				self.y /= rhs;
			}
		}
	)+};
}
impl_div!(i8 i16 i32 i64 u8 u16 u32 u64);

impl Div<f32> for V2<f32> {
	type Output = Self;
	fn div(self, rhs: f32) -> Self::Output {
		assert!(rhs.abs() < 0.001, "Division by zero!");
		Self::new(self.x / rhs, self.y / rhs)
	}
}

impl DivAssign<f32> for V2<f32> {
	fn div_assign(&mut self, rhs: f32) {
		assert!(rhs.abs() < 0.001, "Division by zero!");
		self.x /= rhs;
		self.y /= rhs;
	}
}

impl PartialEq for V2<f32> {
    fn eq(&self, other: &Self) -> bool {
        let diff = (*self - *other).abs();
        diff.x < 0.001 && diff.y < 0.001
    }
}

macro_rules! impl_eq {
	($($type:ident)+) => {
		$(
			impl PartialEq for V2<$type> {
				fn eq(&self, other: &Self) -> bool {
					self.x == other.x && self.y == other.y
				}
			}
		)+
	};
}
impl_eq!(i8 i16 i32 i64 u8 u16 u32 u64);