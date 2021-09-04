use crate::consts::*;
use core::fmt::{Display, Formatter};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    PllInputFrequencyTooLow,
    PllOutputFrequencyTooHigh,
    PllOutputFrequencyTooLow,
    PllInputFrequencyTooHigh,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::PllInputFrequencyTooLow => msg::ERR_PLL_INPUT_FREQ_TOO_LOW,
                Error::PllInputFrequencyTooHigh => msg::ERR_PLL_INPUT_FREQ_TOO_HIGH,
                Error::PllOutputFrequencyTooLow => msg::ERR_PLL_OUTPUT_FREQ_TOO_LOW,
                Error::PllOutputFrequencyTooHigh => msg::ERR_PLL_OUTPUT_FREQ_TOO_HIGH,
            }
        )
    }
}
