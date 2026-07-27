#![no_std]

#[cfg(target_os = "linux")]
pub mod glx;
#[cfg(target_os = "linux")]
pub use self::glx::*;
