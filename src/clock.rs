use crate::pac::PRCI;
use crate::time::Hertz;

const HFXCLK: u32 = 26_000_000;

pub trait PrciExt {
    fn setup(self) -> ClockSetup;
}

impl PrciExt for PRCI {
    fn setup(self) -> ClockSetup {
        ClockSetup {
            prci: self,
            coreclk: None,
        }
    }
}

pub struct ClockSetup {
    prci: PRCI,
    coreclk: Option<u32>,
}

impl ClockSetup {
    pub fn coreclk<F: Into<Hertz>>(mut self, freq: F) -> Self {
        self.coreclk = Some(freq.into().0);
        self
    }

    pub fn freeze(self) -> Clocks {
        let coreclk = self.coreclk.unwrap_or(HFXCLK);

        // Switch to HFXCLK
        unsafe {
            self.prci
                .core_clk_sel_reg
                .modify(|r, w| w.bits(r.bits() | 1));
            self.prci.core_clk_sel_reg.read(); // barrier
        }

        if coreclk == HFXCLK {
            return Clocks {
                coreclk,
                pclk: HFXCLK / 2,
            };
        }

        let divq: u8 = match coreclk {
            1600_000_000..=u32::MAX => panic!("Requested coreclk frequency is too high"),
            f if f >= 1200_000_000 => 1,
            f if f >= 600_000_000 => 2,
            f if f >= 300_000_000 => 3,
            f if f >= 150_000_000 => 4,
            f if f >= 75_000_000 => 5,
            f if f >= 37_500_000 => 6,
            _ => panic!("Requested coreclk frequency is too low"),
        };
        let q = 1 << divq;
        let vco = (coreclk as u64) * (q as u64);

        let divr = (0..3)
            .min_by_key(|divr| {
                let pllin = HFXCLK / (divr + 1);
                let f1 = vco / (2 * pllin as u64);
                let vco1 = f1 * 2 * (pllin as u64);
                ((vco1 as i64) - (vco as i64)).abs()
            })
            .unwrap();
        let pllin = HFXCLK / (divr + 1);
        let divf = (vco / (2 * pllin as u64) - 1) as u16;

        let filter = match pllin {
            f if f < 7_000_000 => panic!("Invalid PLL input frequency"),
            f if f < 11_000_000 => 1,
            f if f < 18_000_000 => 2,
            f if f < 30_000_000 => 3,
            f if f < 50_000_000 => 4,
            f if f < 80_000_000 => 5,
            f if f < 130_000_000 => 6,
            f if f < 200_000_000 => 7,
            _ => panic!("Invalid PLL input frequency"),
        };

        self.prci.core_pllcfg.modify(|_, w| unsafe {
            w.bits(0);
            w.pllr().bits(divr as u8);
            w.pllf().bits(divf);
            w.pllq().bits(divq);
            w.pllrange().bits(filter);
            w.pllbypass().clear_bit();
            w.pllfsebypass().set_bit()
        });

        // Wait for lock
        while self.prci.core_pllcfg.read().plllock().bit_is_clear() {}

        // Switch to PLL
        unsafe {
            self.prci.corepllsel.modify(|r, w| w.bits(r.bits() & !1));
            self.prci.corepllsel.read(); // barrier

            self.prci
                .core_clk_sel_reg
                .modify(|r, w| w.bits(r.bits() & !1));
            self.prci.core_clk_sel_reg.read(); // barrier
        }

        // Calculate real frequency
        let vco = (pllin as u64) * 2 * (divf as u64 + 1);
        let coreclk = (vco >> divq) as u32;
        Clocks {
            coreclk,
            pclk: HFXCLK / 2,
        }
    }
}

pub struct Clocks {
    coreclk: u32,
    pclk: u32,
}

impl Clocks {
    pub fn coreclk(&self) -> Hertz {
        Hertz(self.coreclk)
    }

    pub fn pclk(&self) -> Hertz {
        Hertz(self.pclk)
    }
}
