#![no_std]

#[cfg(all(feature = "embassy", feature = "rtic"))]
compile_error!("Cannot compile for multiple executors. You must choose at most one of `embassy` or `rtic` features.");

pub mod ssq;
pub mod waitqueue;
