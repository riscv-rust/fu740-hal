use crate::error::clock::pll_config::{Error, Result};

pub struct PllConfig {
    pub r: u8,
    pub f: u16,
    pub q: u8,
    pub range: u8,
    pub bypass: bool,
}

impl PllConfig {
    pub fn calculate(input: u32, output: u32) -> Result<PllConfig> {
        if input == output {
            return Ok(PllConfig {
                r: 0,
                f: 0,
                q: 0,
                range: 0,
                bypass: true,
            });
        }

        let divq: u8 = match output {
            f if f > 2_400_000_000 => {
                return Err(Error::OutputFrequencyTooHigh);
            }
            f if f >= 1_200_000_000 => 1,
            f if f >= 600_000_000 => 2,
            f if f >= 300_000_000 => 3,
            f if f >= 150_000_000 => 4,
            f if f >= 75_000_000 => 5,
            f if f >= 37_500_000 => 6,
            _ => {
                return Err(Error::OutputFrequencyTooLow);
            }
        };
        let vco = (output as u64) << divq;

        let divr = (0..3)
            .min_by_key(|divr| {
                let pllin = input / (divr + 1);
                if !(7_000_000..200_000_000).contains(&pllin) {
                    i64::MAX
                } else {
                    let f1 = vco / (2 * pllin as u64);
                    let vco1 = f1 * 2 * (pllin as u64);
                    ((vco1 as i64) - (vco as i64)).abs()
                }
            })
            .unwrap_or_else(|| unreachable!());
        let pllin = input / (divr + 1);
        let divf = (vco / (2 * pllin as u64) - 1) as u16;

        let range = match pllin {
            f if f < 7_000_000 => {
                return Err(Error::InputFrequencyTooLow);
            }
            f if f < 11_000_000 => 1,
            f if f < 18_000_000 => 2,
            f if f < 30_000_000 => 3,
            f if f < 50_000_000 => 4,
            f if f < 80_000_000 => 5,
            f if f < 130_000_000 => 6,
            f if f < 200_000_000 => 7,
            _ => {
                return Err(Error::InputFrequencyTooHigh);
            }
        };

        Ok(PllConfig {
            r: divr as u8,
            f: divf,
            q: divq,
            range,
            bypass: false,
        })
    }

    pub fn output_frequency(&self, input: u32) -> u32 {
        if self.bypass {
            input
        } else {
            let vco = (input as u64) * 2 * (self.f as u64 + 1) / (self.r as u64 + 1);
            (vco >> self.q) as u32
        }
    }
}
