use core::fmt::{Display, Formatter, Result as FmtResult};

use crate::clocks::{CoreclkFreq, PrciSettings};
use crate::freq::Hertz;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    RequestedFrequencyTooHigh(Hertz),
    RequestedFrequencyTooLow(Hertz),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::RequestedFrequencyTooHigh(hertz) => write!(
                f,
                "Requested `coreclk` freq of {} is too high.  Frequency must not exceed {}.",
                hertz,
                CoreclkFreq::max()
            ),
            Error::RequestedFrequencyTooLow(hertz) => write!(
                f,
                "Requested `coreclk` freq of {} is too low.  Frequency must be at least {}.",
                hertz,
                CoreclkFreq::min()
            ),
        }
    }
}
