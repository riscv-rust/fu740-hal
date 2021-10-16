#[cfg(test)]
mod unit_tests;

use super::HertzInner;
use crate::{consts::*, Hertz};

// Create `const` (when supported, for compile-time range-checking) `From` conversions from primitive integer -> `Hertz`
macro_rules! for_hertz {
    ($($ty:ident $($const_kw:ident)?),+) => {
        $(
            impl $($const_kw)? From<$ty> for Hertz {
                fn from(value: $ty) -> Self {
                    // Ensure `HertzInner::MIN` must not overflow `$ty` (or range check (below) will not work as
                    //        intended).  Minimum Hertz is expected to be zero.
                    assert!(HertzInner::MIN == 0, "{}", msg::ERR_INVALID_HERTZ_INNER_TYPE);

                    // Min-range-check `value`
                    assert!(value >= HertzInner::MIN as $ty, "{}", msg::ERR_HERTZ_BELOW_MINIMUM);

                    // If the current `From` type has more bits than `HertzInner`, ensure no overflow
                    match $ty::BITS > HertzInner::BITS {
                        // Overflow is possible: Max-range-check value
                        true => assert!(value <= HertzInner::MAX as $ty, "{}", msg::ERR_HERTZ_INNER_OVERFLOW),

                        // Overflow is not possible; safe to continue
                        false => (),
                    }

                    #[allow(clippy::as_conversions)]
                    Self(value as HertzInner)
                }
            }
        )+
    }
}

// When `const_trait_impl` is available, use `impl const From<T: PrimInt> for Hertz`
#[cfg(any(feature = "const_conversions", feature = "const_trait_impl"))]
for_hertz!(i8 const, i16 const, i32 const, i64 const, i128 const, isize const,
           u8 const, u16 const, u32 const, u64 const, u128 const, usize const);

// When `const_trait_impl` is not available, use `impl From<T: PrimInt> for Hertz` (note lack of `const` trait impl)
#[cfg(not(any(feature = "const_conversions", feature = "const_trait_impl")))]
for_hertz!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
