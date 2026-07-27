#![no_std]

pub mod vk;
pub use self::vk::*;
#[cfg(feature = "alloc")]
extern crate alloc;
