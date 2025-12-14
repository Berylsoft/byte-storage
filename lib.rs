#![deny(unused_results)]

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String, vec::Vec};

#[cfg(feature = "bytes")]
pub use bytes::{self, Bytes};

mod marker;
pub use marker::*;

#[cfg(feature = "str")]
mod bytestr;
#[cfg(feature = "str")]
pub use bytestr::*;

mod input;
pub use input::*;

mod output;
pub use output::*;
