mod prci_ext;
mod prci_settings;
use crate::{
    error::frequency::Error,
    freq::{Hertz},
};
pub use {
    prci_ext::PrciExt,
    prci_settings::{CoreclkFreq, PrciSettings},
};

// Per §7.4.1 & §7.4.2-1 https://sifive.cdn.prismic.io/sifive/18febb04-50b6-4880-9bf3-631e40daa809_fu740-c000-manual-v1p2.pdf
const HFXCLK: u32 = 26_000_000;

#[derive(Debug)]
struct PllConfig {
    r: u8,
    f: u16,
    q: u8,
    range: u8,
    bypass: bool,
}

impl PllConfig {
   fn calculate_coreclk_pll(input: CoreclkFreq, output: CoreclkFreq) -> PllConfig {
    //     if input == output {
    //         return PllConfig {
    //             r: 0,
    //             f: 0,
    //             q: 0,
    //             range: 0,
    //             bypass: true,
    //         };
    //     }
    //
    //     let divq: u8 = match output.hz() {
    //         f if f > CoreclkFreq::max() => {
    //             unreachable!("Internal error: `output: CoreclkFreq` contains value higher than `CoreclkFreq::max()`.")
    //         }
    //         f if f >= 1_200_000_000.hz() => 1,
    //         f if f >= 600_000_000.hz() => 2,
    //         f if f >= 300_000_000.hz() => 3,
    //         f if f >= 150_000_000.hz() => 4,
    //         f if f >= 75_000_000.hz() => 5,
    //         f if f >= CoreclkFreq::min() => 6,
    //         _ => unreachable!("Internal error: `output: CoreclkFreq` contains a value lower than `CoreclkFreq::min()`."),
    //     };
        // let output_u64 = output
        //     .hz()
        //     .as_checked_u64()
        //     .ok_or_else(|| Error::ToU64Overflow(output.hz()))
        //     .unwrap_or_else(|err| {
        //         unreachable!("Internal error: `output.hz()` overflowed `u64`: {}.", err)
        //     });
        //
        // let vco = output_u64 << divq;
        //
        // let input_u64 = input
        //     .hz()
        //     .as_checked_u64()
        //     .ok_or_else(|| Error::ToU64Overflow(input.hz()))
        //     .unwrap_or_else(|err| {
        //         unreachable!("Internal error: `input.hz()` overflowed `u64`: {}.", err)
        //     });
        //
        // let divr = (0..3)
        //     .min_by_key(|divr| {
        //         let pllin = input_u64 / (divr + 1);
        //         if !(7_000_000..200_000_000).contains(&pllin) {
        //             i64::MAX
        //         } else {
        //             let f1 = vco / (2 * pllin as u64);
        //             let vco1 = f1 * 2 * (pllin as u64);
        //             ((vco1 as i64) - (vco as i64)).abs()
        //         }
        //     })
        //     .unwrap_or_else(|| {
        //         unreachable!(
        //             "Internal error: `min_by_key()` returned `None` from non-empty iterator."
        //         )
        //     });
        //
        // let pllin = input_u64 / (divr + 1);
        // let divf = (vco / (2 * pllin as u64) - 1) as u16;
        //
        // let range = match pllin {
        //     f if f < 7_000_000 => {
        //         unreachable!("Internal error: `input CoreclkFreq` contains an illegal value.")
        //     }
        //     f if f < 11_000_000 => 1,
        //     f if f < 18_000_000 => 2,
        //     f if f < 30_000_000 => 3,
        //     f if f < 50_000_000 => 4,
        //     f if f < 80_000_000 => 5,
        //     f if f < 130_000_000 => 6,
        //     f if f < 200_000_000 => 7,
        //     _ => unreachable!("Internal error: `input CoreclkFreq` contains an illegal value."),
        // };
        //
        // PllConfig {
        //     r: divr as u8,
        //     f: divf,
        //     q: divq,
        //     range,
        //

        unimplemented!()
    }

    fn output_frequency(&self, input: CoreclkFreq) -> CoreclkFreq {
        if self.bypass {
            input
        } else {
            // let input_u64 = input
            //     .hz()
            //     .as_checked_u64()
            //     .ok_or_else(|| Error::ToU64Overflow(input.hz()))
            //     .unwrap_or_else(|err| {
            //         unreachable!("Internal error: `input.hz()` overflowed `u64`: {}.", err)
            //     });
            // let vco = input_u64
            //     .checked_mul(2)
            //     .unwrap()
            //     .checked_mul((u64::from(self.f)).checked_add(1).unwrap())
            //     .unwrap()
            //     .checked_div((u64::from(self.r)).checked_add(1).unwrap())
            //     .unwrap();
            // (vco >> self.q).hz().into()
            unimplemented!()
        }
    }
}

#[derive(Debug)]
pub struct ClockFreqs {
    coreclk: CoreclkFreq,
    hfpclk: Hertz,
}

impl ClockFreqs {
    pub fn coreclk(&self) -> CoreclkFreq {
        self.coreclk
    }

    pub fn hfpclk(&self) -> Hertz {
        self.hfpclk
    }
}
