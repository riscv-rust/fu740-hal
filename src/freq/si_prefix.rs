use core::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum SiPrefix {
    Kilo,
    Mega,
    Giga,
}

impl Display for SiPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use SiPrefix::*;
        write!(
            f,
            "{}",
            match self {
                Kilo => "kilo",
                Mega => "mega",
                Giga => "giga",
            }
        )
    }
}

impl From<SiPrefix> for u32 {
    fn from(si_prefix: SiPrefix) -> Self {
        use SiPrefix::*;
        match si_prefix {
            Kilo => 1_000,
            Mega => 1_000_000,
            Giga => 1_000_000_000,
        }
    }
}

impl From<SiPrefix> for u64 {
    fn from(si_prefix: SiPrefix) -> Self {
        use SiPrefix::*;
        match si_prefix {
            Kilo => 1_000,
            Mega => 1_000_000,
            Giga => 1_000_000_000,
        }
    }
}
