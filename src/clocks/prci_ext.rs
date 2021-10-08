use super::PrciSettings;
use crate::pac::PRCI;

pub trait PrciExt {
    fn setup(self) -> PrciSettings;
}

impl PrciExt for PRCI {
    fn setup(self) -> PrciSettings {
        PrciSettings::new(self)
    }
}
