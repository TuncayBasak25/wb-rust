use super::V2;

macro_rules! impl_conversion {
	($self:ident $($other:ident)*) => {
		impl_conversion!($($other)*);
		$(
			impl From<V2<$other>> for V2<$self> {
				fn from(other: V2<$other>) -> Self {
					Self::new(other.x as $self, other.y as $self)
				}
			}
			
			impl From<V2<$self>> for V2<$other> {
				fn from(other: V2<$self>) -> Self {
					Self::new(other.x as $other, other.y as $other)
				}
			}
		)*
	};

	(impl $self:ident) => {};
	() => {};
}
impl_conversion!(i8 i16 i32 i64 u8 u16 u32 u64 f32);