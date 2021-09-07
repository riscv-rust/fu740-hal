pub mod pll_config;

use crate::{concat_str, consts::*};
use core::fmt::{Display, Formatter, Result as FmtResult};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    CorePllError(pll_config::Error),
    HfpClkPllError(pll_config::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut buf = [0u8; DEFAULT_CONCAT_STR_BUFFER_SIZE_IN_BYTES];

        write!(
            f,
            "{}",
            match self {
                Error::CorePllError(error) =>
                    concat_str::show(&mut buf, format_args!("CorePllError: {}", error))?,
                Error::HfpClkPllError(error) =>
                    concat_str::show(&mut buf, format_args!("HfpClkPllError: {}", error))?,
            }
        )
    }
}
