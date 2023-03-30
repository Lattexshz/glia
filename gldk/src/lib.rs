use core::ffi::c_void;
use core::fmt::Arguments;
pub(crate) use platform_impl::*;

pub mod window;
mod platform_impl;

