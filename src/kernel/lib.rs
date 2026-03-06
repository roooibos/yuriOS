#![no_std]
#![feature(abi_x86_interrupt)]
#![allow(unused)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

/// カーネル本体
pub mod kernel;