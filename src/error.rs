pub mod clock;

use crate::{concat_str, consts::*};
use core::fmt::{Display, Formatter, Result as FmtResult};

#[allow(dead_code)]
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    ClockError(clock::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut buf = [0u8; DEFAULT_CONCAT_STR_BUFFER_SIZE_IN_BYTES];

        write!(
            f,
            "{}",
            match self {
                Error::ClockError(error) =>
                    concat_str::show(&mut buf, format_args!("ClockError: {}", error))?,
            }
        )
    }
}

impl From<clock::Error> for Error {
    fn from(error: clock::Error) -> Self {
        Self::ClockError(error)
    }
}
