use crate::freq::Hertz;

pub struct Coreclk(Hertz);

impl Coreclk {
    // The following limits represent the limits of what is possible to be encoded into the PLLs.
    // Please take care not to exceed the clock freq limits specified by the reference manual.
    // fu740 Reference: https://sifive.cdn.prismic.io/sifive/18febb04-50b6-4880-9bf3-631e40daa809_fu740-c000-manual-v1p2.pdf
    // Per §7.1
    pub const MIN: u64 =    37_500_000;
    pub const MAX: u64 = 2_400_000_000;

    const fn new(hz: Hertz) -> Self {
        assert!(hz.as_checked_u64().unwrap() >= Self::MIN, "Coreclk frequency is below supported minimum.");
        assert!(hz.as_checked_u64().unwrap() <= Self::MAX, "Coreclk frequency is above supported maximum.");

        Self(hz)
    }
}

#[cfg(any(feature = "const_conversions", feature = "const_trait_impl"))]
impl const From<Hertz> for Coreclk {
    fn from(hz: Hertz) -> Self {
        Self::new(hz)
    }
}

#[cfg(not(any(feature = "const_conversions", feature = "const_trait_impl")))]
impl From<Hertz> for Coreclk {
    fn from(hz: Hertz) -> Self {
        Self::new(hz)
    }
}
