#![cfg_attr(feature = "const_conversions", feature(const_fn_trait_bound, const_option, const_trait_impl))]
#![no_std]
pub mod clocks;
mod consts;
pub mod delay;
mod error;
pub mod prelude;
pub mod serial;
pub mod stdout;
mod freq;

pub use {error::{Error, Result}, freq::Hertz};
pub use fu740_pac as pac;
