mod pll_config;

use crate::{
    clock::pll_config::PllConfig,
    error::clock::{Error, Result},
    pac::PRCI,
    time::Hertz,
};

const HFXCLK: u32 = 26_000_000;

pub trait PrciExt {
    fn setup(self) -> ClockSetup;
}

impl PrciExt for PRCI {
    fn setup(self) -> ClockSetup {
        ClockSetup {
            prci: self,
            coreclk: None,
            pclk: None,
        }
    }
}

pub struct ClockSetup {
    prci: PRCI,
    coreclk: Option<u32>,
    pclk: Option<u32>,
}

impl ClockSetup {
    pub fn coreclk<F: Into<Hertz>>(mut self, freq: F) -> Self {
        let freq = freq.into().0;
        assert!(freq < 1_600_000_000);

        self.coreclk = Some(freq);
        self
    }

    pub fn freeze(self) -> Clocks {
        self.try_freeze().unwrap()
    }

    pub fn pclk<F: Into<Hertz>>(mut self, freq: F) -> Self {
        let freq = freq.into().0;
        assert!(freq < 125_000_000);

        self.pclk = Some(freq);
        self
    }

    pub fn try_freeze(self) -> Result<Clocks> {
        let coreclk = self.coreclk.unwrap_or(HFXCLK);
        let pclk = self.pclk.unwrap_or(HFXCLK / 2);

        let core_pll = PllConfig::calculate(HFXCLK, coreclk).map_err(Error::CorePllError)?;
        let hfpclk_pll = PllConfig::calculate(HFXCLK, pclk * 2).map_err(Error::HfpClkPllError)?;

        unsafe {
            // Switch core clock to HFXCLK
            self.prci.core_clk_sel_reg.modify(|_, w| w.source().hfclk());

            // Apply PLL configuration
            self.prci.core_pllcfg.write_with_zero(|w| {
                w.pllr().bits(core_pll.r);
                w.pllf().bits(core_pll.f);
                w.pllq().bits(core_pll.q);
                w.pllrange().bits(core_pll.range);
                w.pllbypass().bit(core_pll.bypass);
                w.pllfsebypass().set_bit()
            });

            if !core_pll.bypass {
                // Wait for lock
                while self.prci.core_pllcfg.read().plllock().bit_is_clear() {}

                // Select corepll
                self.prci.corepllsel.modify(|_, w| w.source().corepll());
            }

            if coreclk != HFXCLK {
                // Select PLL as a core clock source
                self.prci
                    .core_clk_sel_reg
                    .modify(|_, w| w.source().pll_mux());
            }

            // Switch peripheral clock to HFXCLK
            self.prci.hfpclkpllsel.modify(|_, w| w.source().hfclk());

            // Apply PLL configuration
            self.prci.hfpclk_pllcfg.write_with_zero(|w| {
                w.pllr().bits(hfpclk_pll.r);
                w.pllf().bits(hfpclk_pll.f);
                w.pllq().bits(hfpclk_pll.q);
                w.pllrange().bits(hfpclk_pll.range);
                w.pllbypass().bit(hfpclk_pll.bypass);
                w.pllfsebypass().set_bit()
            });

            if !hfpclk_pll.bypass {
                // Wait for lock
                while self.prci.hfpclk_pllcfg.read().plllock().bit_is_clear() {}
            }

            // Enable clock
            self.prci
                .hfpclk_plloutdiv
                .modify(|r, w| w.bits(r.bits() | 1u32 << 31));

            if pclk != HFXCLK / 2 {
                // Select PLL as a peripheral clock source
                self.prci.hfpclkpllsel.modify(|_, w| w.source().hfpclkpll());
            }

            // Set divider to 0 (divide by 2)
            self.prci.hfpclk_div_reg.write_with_zero(|w| w.bits(0));
        }

        Ok(Clocks {
            coreclk: core_pll.output_frequency(HFXCLK),
            pclk: hfpclk_pll.output_frequency(HFXCLK) / 2,
        })
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
