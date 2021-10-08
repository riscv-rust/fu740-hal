use crate::{clocks::HFXCLK, freq::Hertz};
use num_traits::PrimInt;

pub trait Freq<I: PrimInt>: Default {
    const DEFAULT: I;
    const RANGE: (I, I);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct CoreclkFreq(Hertz);

impl CoreclkFreq {
    // The following limits represent the limits of what is possible to be encoded into the PLLs.
    // Please take care not to exceed the clock freq limits specified by the reference manual.
    // fu740 Reference: https://sifive.cdn.prismic.io/sifive/18febb04-50b6-4880-9bf3-631e40daa809_fu740-c000-manual-v1p2.pdf
    // Per §7.1
    pub(crate) const CORECLK_DEFAULT: u32 = HFXCLK;
    pub(crate) const CORECLK_RANGE: (u32, u32) = (37_500_000, 2_400_000_000);

    const fn new(hz: Hertz) -> Self {
        assert!(Self::CORECLK_RANGE.0 <= Self::CORECLK_RANGE.1);
        assert!(
            Self::CORECLK_DEFAULT >= Self::CORECLK_RANGE.0
                && Self::CORECLK_DEFAULT <= Self::CORECLK_RANGE.1
        );
        Self(hz)
    }

    pub const fn hz(&self) -> Hertz {
        self.0
    }

    pub const fn max() -> Hertz {
        // Self::CORECLK_RANGE.1.hz()
        unimplemented!()
    }

    pub const fn min() -> Hertz {
        // Self::CORECLK_RANGE.0.hz()
        unimplemented!()
    }
}

impl Default for CoreclkFreq {
    fn default() -> Self {
        unimplemented!()

        // Self(Self::CORECLK_DEFAULT.hz())
    }
}

impl From<Hertz> for CoreclkFreq {
    fn from(hz: Hertz) -> Self {
        unimplemented!()
        // Self::new(hz)
    }
}

impl From<CoreclkFreq> for Hertz {
    fn from(freq: CoreclkFreq) -> Self {
        unimplemented!()

        // freq.0
    }
}
