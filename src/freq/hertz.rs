mod from_prim_int;
mod prim_int_ext;

use crate::consts::*;
use core::fmt::{Display, Formatter, Result as FmtResult};
pub use prim_int_ext::PrimIntExt;

type HertzInner = u64;

/// Hertz
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Hertz(HertzInner);

impl Hertz {
    pub const fn as_checked_u32(&self) -> Option<u32> {
        match HertzInner::BITS <= u32::BITS {
            true => Some(self.0 as u32),
            false => match self.0 {
                value if value <= u32::MAX as HertzInner => Some(value as u32),
                _ => None,
            },
        }
    }

    pub const fn as_checked_u64(&self) -> Option<u64> {
        match HertzInner::BITS <= u64::BITS {
            true => Some(self.0 as u64),
            false => match self.0 {
                value if value <= u64::MAX as HertzInner => Some(value as u64),
                _ => None,
            },
        }
    }
}

impl Display for Hertz {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {}", self.0, msg::HERTZ)
    }
}
