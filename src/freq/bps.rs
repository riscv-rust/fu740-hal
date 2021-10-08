use crate::freq::Hertz;

pub struct Bps(Hertz);

impl Bps {
    // The following limits represent the limits of what is possible to be encoded into the PLLs.
    // Please take care not to exceed the clock freq limits specified by the reference manual.
    // fu740 Reference: https://sifive.cdn.prismic.io/sifive/18febb04-50b6-4880-9bf3-631e40daa809_fu740-c000-manual-v1p2.pdf
    // Per §7.1
    // TODO: Update limits for Bps, cite reference
    pub const MIN: u64 =         1;
    pub const MAX: u64 = 1_000_000;

    #[cfg(feature = "const-conversions")]
    const fn new(hz: Hertz) -> Self {
        assert!(hz.as_checked_u64().unwrap() >= Self::MIN);
        assert!(hz.as_checked_u64().unwrap() <= Self::MAX);

        Self(hz)
    }

    #[cfg(not(feature = "const-conversions"))]
    fn new(hz: Hertz) -> Self {
        assert!(hz.as_checked_u64().expect("Coreclk frequency exceeds supported maximum") >= Self::MIN);
        assert!(hz.as_checked_u64().expect("Coreclk frequency exceeds supported maximum") <= Self::MAX);

        Self(hz)
    }
}


#[cfg(feature = "const_trait_impl")]
impl const From<Hertz> for Bps {
    fn from(hz: Hertz) -> Self {
        Self::new(hz)
    }
}

#[cfg(not(feature = "const_trait_impl"))]
impl From<Hertz> for Bps {
    fn from(hz: Hertz) -> Self {
        Self::new(hz)
    }
}
