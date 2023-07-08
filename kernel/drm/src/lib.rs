#[macro_use]
extern crate bitflags;

extern crate libc;

pub mod core;
mod ffi;

pub use ffi::ConnectionStatus;
