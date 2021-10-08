use crate::Hertz;

pub trait PrimIntExt
    where
        Self: Into<Hertz>,
{
    fn hz(self) -> Hertz { self.into() }
}
