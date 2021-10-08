use num_traits::PrimInt;

pub trait ICoreclkHz {
    type Inner: PrimInt;
    type Output;
    const MIN: Self::Inner;
    const MAX: Self::Inner;

    fn coreclk_hz(self) -> Self::Output;
}
