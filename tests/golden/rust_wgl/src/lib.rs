#![no_std]

#[cfg(windows)]
pub mod wgl;
#[cfg(windows)]
pub use self::wgl::*;
