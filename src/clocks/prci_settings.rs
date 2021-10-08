mod coreclk_freq;

use super::{ClockFreqs, PllConfig, HFXCLK};
pub use crate::clocks::prci_settings::coreclk_freq::CoreclkFreq;
use crate::pac::PRCI;

#[derive(Debug)]
pub struct PrciSettings {
    prci: PRCI,
    coreclk_freq: CoreclkFreq,
    ddrctrlclk_freq: Option<u32>,
    hfpclk_freq: Option<u32>,
}

impl PrciSettings {
    pub(crate) fn new(prci: PRCI) -> Self {
        Self {
            prci,
            coreclk_freq: CoreclkFreq::default(),
            ddrctrlclk_freq: None,
            hfpclk_freq: None,
        }
    }

    pub fn set_coreclk_freq<F: Into<CoreclkFreq>>(mut self, freq: F) -> Self {
        let freq = freq.into();

        self.coreclk_freq = freq;
        self
    }

    // pub fn set_ddrctrlclk<F: Into<Hertz>>(mut self, freq: F) -> Self {
    //     let freq = freq.into().0;
    //     assert!(freq <= Self::DDRCTRLCLK_RANGE.1);
    //
    //     self.ddrctrlclk_freq = Some(freq);
    //     self
    // }

    // pub fn set_hfpclk<F: Into<Hertz>>(mut self, freq: F) -> Self {
    //     let freq = freq.into().0;
    //     assert!(freq <= Self::HFPCLK_RANGE.1);
    //
    //     self.hfpclk_freq = Some(freq);
    //     self
    // }

    pub fn freeze(self) -> ClockFreqs {
        // let core_pll = PllConfig::calculate_coreclk_pll(HFXCLK.hz().into(), self.coreclk_freq);
        // // Switch core clock to HFXCLK
        // self.prci.core_clk_sel_reg.modify(|_, w| w.source().hfclk());
        //
        // // Apply PLL configuration
        // unsafe {
        //     self.prci.core_pllcfg.write_with_zero(|w| {
        //         w.pllr().bits(core_pll.r);
        //         w.pllf().bits(core_pll.f);
        //         w.pllq().bits(core_pll.q);
        //         w.pllrange().bits(core_pll.range);
        //         w.pllbypass().bit(core_pll.bypass);
        //         w.pllfsebypass().set_bit()
        //     });
        // }
        //
        // if !core_pll.bypass {
        //     // Wait for lock
        //     while self.prci.core_pllcfg.read().plllock().bit_is_clear() {}
        //
        //     // Select corepll
        //     self.prci.corepllsel.modify(|_, w| w.source().corepll());
        // }
        //
        // if
        // /* freq.into() */
        // HFXCLK != HFXCLK {
        //     // Select PLL as a core clocks source
        //     self.prci
        //         .core_clk_sel_reg
        //         .modify(|_, w| w.source().pll_mux());
        // }
        //
        // //        let ddrctrlclk = self.ddrctrlclk.unwrap_or(Self::DDRCTRLCLK_RANGE);
        // let hfpclk = self.hfpclk_freq.unwrap_or(HFXCLK / 2);
        //
        // let hfpclk_pll = PllConfig::calculate_coreclk_pll(HFXCLK.hz().into(), (hfpclk * 2).hz().into());
        //
        // // Switch peripheral clocks to HFXCLK
        // self.prci.hfpclkpllsel.modify(|_, w| w.source().hfclk());
        //
        // // Apply PLL configuration
        // unsafe {
        //     self.prci.hfpclk_pllcfg.write_with_zero(|w| {
        //         w.pllr().bits(hfpclk_pll.r);
        //         w.pllf().bits(hfpclk_pll.f);
        //         w.pllq().bits(hfpclk_pll.q);
        //         w.pllrange().bits(hfpclk_pll.range);
        //         w.pllbypass().bit(hfpclk_pll.bypass);
        //         w.pllfsebypass().set_bit()
        //     });
        // }
        //
        // if !hfpclk_pll.bypass {
        //     // Wait for lock
        //     while self.prci.hfpclk_pllcfg.read().plllock().bit_is_clear() {}
        // }
        //
        // // Enable clocks
        // self.prci
        //     .hfpclk_plloutdiv
        //     .modify(|r, w| unsafe { w.bits(r.bits() | 1u32 << 31) });
        //
        // if hfpclk != HFXCLK / 2 {
        //     // Select PLL as a peripheral clocks source
        //     self.prci.hfpclkpllsel.modify(|_, w| w.source().hfpclkpll());
        // }
        //
        // // Set divider to 0 (divide by 2)
        // unsafe {
        //     self.prci.hfpclk_div_reg.write_with_zero(|w| w.bits(0));
        // }

        // ClockFreqs {
        //     coreclk: core_pll.output_frequency(HFXCLK.hz().into()),
        //     hfpclk: (hfpclk_pll.output_frequency(HFXCLK.hz().into()).hz().as_checked_u64().unwrap() / 2).hz().into(),
        // }
        unimplemented!()
    }
}
