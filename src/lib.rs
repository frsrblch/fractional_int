macro_rules! fractional_int {
    ($i:ident, $inner:ident) => {
        #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $i($inner);

        impl $i {
            pub const MAX: Self = Self::new(<$inner>::MAX);

            pub const fn new(value: $inner) -> Self {
                Self(value)
            }

            pub fn new_f32(value: f32) -> Self {
                const MAX: f32 = <$inner>::MAX as f32;
                Self((value * MAX) as $inner)
            }

            pub fn new_f64(value: f64) -> Self {
                const MAX: f64 = <$inner>::MAX as f64;
                Self((value * MAX) as $inner)
            }

            pub fn inverse(self) -> Self {
                Self::new(<$inner>::MAX - self.0)
            }

            pub fn $inner(self) -> $inner {
                self.0
            }

            pub fn f32(self) -> f32 {
                const MAX_INV: f32 = 1.0 / <$inner>::MAX as f32;
                self.0 as f32 * MAX_INV
            }

            pub fn f64(self) -> f64 {
                const MAX_INV: f64 = 1.0 / <$inner>::MAX as f64;
                self.0 as f64 * MAX_INV
            }

            pub fn max(self, rhs: Self) -> Self {
                Self(self.0.max(rhs.0))
            }

            pub fn min(self, rhs: Self) -> Self {
                Self(self.0.min(rhs.0))
            }
        }

        impl From<$inner> for $i {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl std::ops::Add for $i {
            type Output = $i;
            fn add(self, rhs: Self) -> Self {
                Self(self.0.saturating_add(rhs.0))
            }
        }

        impl std::ops::Add<$inner> for $i {
            type Output = $i;
            fn add(self, rhs: $inner) -> Self {
                Self(self.0.saturating_add(rhs))
            }
        }

        impl std::ops::AddAssign for $i {
            fn add_assign(&mut self, rhs: Self) {
                self.0 = self.0.saturating_add(rhs.0);
            }
        }

        impl std::ops::AddAssign<$inner> for $i {
            fn add_assign(&mut self, rhs: $inner) {
                self.0 = self.0.saturating_add(rhs);
            }
        }

        impl std::ops::Sub for $i {
            type Output = $i;
            fn sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }
        }

        impl std::ops::Sub<$inner> for $i {
            type Output = $i;
            fn sub(self, rhs: $inner) -> Self {
                Self(self.0.saturating_sub(rhs))
            }
        }

        impl std::ops::SubAssign for $i {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 = self.0.saturating_sub(rhs.0);
            }
        }

        impl std::ops::SubAssign<$inner> for $i {
            fn sub_assign(&mut self, rhs: $inner) {
                self.0 = self.0.saturating_sub(rhs);
            }
        }

        impl std::ops::Not for $i {
            type Output = Self;
            fn not(self) -> Self {
                self.inverse()
            }
        }
    };
}

fractional_int!(FractionalU8, u8);
fractional_int!(FractionalU16, u16);

impl FractionalU8 {
    pub fn u16(self) -> FractionalU16 {
        FractionalU16::new(self.0 as u16 * 257)
    }
}

impl FractionalU16 {
    pub fn u8(self) -> FractionalU8 {
        FractionalU8::new((self.0 / 257) as u8)
    }
}

impl std::ops::Mul for FractionalU8 {
    type Output = FractionalU16;

    fn mul(self, rhs: Self) -> Self::Output {
        FractionalU16::new_f64(self.f64() * rhs.f64())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f32_lt_zero_returns_zero() {
        assert_eq!(FractionalU8::default(), FractionalU8::new_f32(-0.1));
    }

    #[test]
    fn f64_lt_zero_returns_zero() {
        assert_eq!(FractionalU8::default(), FractionalU8::new_f64(-0.1));
    }

    #[test]
    fn f32_gt_one_returns_one() {
        assert_eq!(FractionalU8::MAX, FractionalU8::new_f32(1.1));
    }

    #[test]
    fn f64_gt_one_returns_one() {
        assert_eq!(FractionalU8::MAX, FractionalU8::new_f64(1.1));
    }

    #[test]
    fn zero_is_zero_f32() {
        assert_eq!(0.0, FractionalU8::new(0).f32());
        assert_eq!(0.0, FractionalU16::new(0).f32());
    }

    #[test]
    fn zero_is_zero_f64() {
        assert_eq!(0.0, FractionalU8::new(0).f64());
        assert_eq!(0.0, FractionalU16::new(0).f64());
    }

    #[test]
    fn max_is_one_f32() {
        assert_eq!(1.0, FractionalU8::MAX.f32());
        assert_eq!(1.0, FractionalU16::MAX.f32());
    }

    #[test]
    fn max_is_one_f64() {
        assert_eq!(1.0, FractionalU8::MAX.f64());
        assert_eq!(1.0, FractionalU16::MAX.f64());
    }

    #[test]
    fn new_f64_zero() {
        assert_eq!(0, FractionalU8::new_f64(0.0).0);
        assert_eq!(0, FractionalU16::new_f64(0.0).0);
    }

    #[test]
    fn new_f64_one() {
        assert_eq!(u8::MAX, FractionalU8::new_f64(1.0).0);
        assert_eq!(u16::MAX, FractionalU16::new_f64(1.0).0);
    }

    #[test]
    fn new_f64_half() {
        assert_eq!(u8::MAX / 2, FractionalU8::new_f64(0.5).0);
        assert_eq!(u16::MAX / 2, FractionalU16::new_f64(0.5).0);
    }

    #[test]
    fn u8_to_u16() {
        let zero = FractionalU8::new(0);
        let one = FractionalU8::new(u8::MAX);

        assert_eq!(0, zero.u16().0);
        assert_eq!(u16::MAX, one.u16().0);
    }

    #[test]
    fn u16_to_u8() {
        let zero = FractionalU16::new(0);
        let one = FractionalU16::new(u16::MAX);

        assert_eq!(0, zero.u8().0);
        assert_eq!(u8::MAX, one.u8().0);
    }

    #[test]
    fn u8_mul_to_u16() {
        assert_eq!(
            FractionalU16::new(0),
            FractionalU8::new(0) * FractionalU8::new(0)
        );

        assert_eq!(
            FractionalU16::new_f32(0.0),
            FractionalU8::new_f32(0.0) * FractionalU8::new_f32(1.0)
        );

        assert_eq!(
            FractionalU16::new_f32(0.0),
            FractionalU8::new_f32(1.0) * FractionalU8::new_f32(0.0)
        );

        assert_eq!(
            FractionalU16::new_f32(1.0),
            FractionalU8::new_f32(1.0) * FractionalU8::new_f32(1.0)
        );

        assert_eq!(
            FractionalU16::new_f32(0.24805), // rounding error
            FractionalU8::new_f32(0.5) * FractionalU8::new_f32(0.5)
        );
    }
}
