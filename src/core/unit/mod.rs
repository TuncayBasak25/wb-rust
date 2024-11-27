pub mod rad;
#[macro_export]
macro_rules! define_unit {
    ($t:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $t(f32);

        impl $t {
            pub fn val(&self) -> f32 {
                self.0
            }

            pub fn add(&mut self, rhs: $t) -> &mut Self {
                self.0 += rhs.val();
                self
            }

            pub fn sub(&mut self, rhs: $t) -> &mut Self {
                self.0 -= rhs.val();
                self
            }
        }

        impl std::ops::Add<$t> for $t {
            type Output = $t;

            fn add(self, rhs: $t) -> Self {
                Self(self.val() + rhs.val())
            }
        }

        impl std::ops::Sub<$t> for $t {
            type Output = $t;

            fn sub(self, rhs: $t) -> Self {
                Self(self.val() - rhs.val())
            }
        }

        impl std::ops::AddAssign<$t> for $t {
            fn add_assign(&mut self, rhs: $t) {
                self.0 += rhs.val();
            }
        }

        impl std::ops::SubAssign<$t> for $t {
            fn sub_assign(&mut self, rhs: $t) {
                self.0 -= rhs.val();
            }
        }
    };
}