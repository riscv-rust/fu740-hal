use crate::time::Hertz;
use crate::pac::PRCI;

const HFXCLK: u32 = 26_000_000;

pub trait PrciExt {
    fn setup(self) -> ClockSetup;
}

impl PrciExt for PRCI {
    fn setup(self) -> ClockSetup {
        ClockSetup {
            prci: self,
        }
    }
}

pub struct ClockSetup {
    #[allow(dead_code)]
    prci: PRCI,
}

impl ClockSetup {
    pub fn freeze(self) -> Clocks {
        Clocks {
            coreclk: HFXCLK
        }
    }
}

pub struct Clocks {
    coreclk: u32
}

impl Clocks {
    pub fn coreclk(&self) -> Hertz {
        Hertz(self.coreclk)
    }

    pub fn tlclk(&self) -> Hertz {
        Hertz(self.coreclk / 2)
    }
}
