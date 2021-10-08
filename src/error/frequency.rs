use crate::freq::{Hertz, SiPrefix};
use core::fmt::{Display, Formatter, Result as FmtResult};
use core::num::TryFromIntError;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    I32ToHzOverflow(SiPrefix, i32),
    U32ToHzOverflow(SiPrefix, u32),
    U64ToHzOverflow(SiPrefix, u64),
    ToU32Overflow(Hertz),
    ToU64Overflow(Hertz),
    NegativeFrequency(i32, TryFromIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Error::*;

        match self {
            I32ToHzOverflow(si_prefix, freq) => {
                write!(
                    f,
                    "Error: Overflow converting {} {} (x{}) to Hertz.",
                    freq,
                    si_prefix,
                    u64::from(*si_prefix)
                )
            },
            U32ToHzOverflow(si_prefix, freq) => {
                write!(
                    f,
                    "Error: Overflow converting {} {} (x{}) to Hertz.",
                    freq,
                    si_prefix,
                    u64::from(*si_prefix)
                )
            },
            U64ToHzOverflow(si_prefix, freq) => {
                write!(
                    f,
                    "Error: Overflow converting {} {} (x{}) to Hertz.",
                    freq,
                    si_prefix,
                    u64::from(*si_prefix)
                )
            },
            ToU32Overflow(hz) => write!(
                f,
                "Error: Overflow representing `Hertz` value as a `u32`: {}.",
                hz
            ),
            ToU64Overflow(hz) => write!(
                f,
                "Error: Overflow representing `Hertz` value as a `u64`: {}.",
                hz
            ),
            NegativeFrequency(freq, err) => write!(
                f,
                "Error: Negative freq requested.  Frequency must be non-negative: {}. ({})",
                freq, err
            ),
        }
    }
}
