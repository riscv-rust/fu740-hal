pub mod coreclk;
pub mod frequency;

use crate::freq::SiPrefix;
use core::fmt::{Display, Formatter, Result as FmtResult};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    CoreclkError(coreclk::Error),
    ToHzOverflow(SiPrefix, u64),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Error::*;

        match self {
            CoreclkError(inner) => write!(f, "{}", inner),
            ToHzOverflow(si_prefix, freq) => {
                write!(
                    f,
                    "Error: Overflow converting {} {}hertz (x{}) to hertz.",
                    freq,
                    si_prefix,
                    u64::from(*si_prefix)
                )
            }
        }
    }
}
