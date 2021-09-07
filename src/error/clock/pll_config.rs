use core::fmt::{Display, Formatter, Result as FmtResult};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    InputFrequencyTooHigh,
    InputFrequencyTooLow,
    OutputFrequencyTooHigh,
    OutputFrequencyTooLow,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Error::InputFrequencyTooHigh => "Provided PLL input frequency is too high",
                Error::InputFrequencyTooLow => "Provided PLL input frequency is too low",
                Error::OutputFrequencyTooHigh => "Requested PLL output frequency is too high",
                Error::OutputFrequencyTooLow => "Requested PLL output frequency is too low",
            }
        )
    }
}
