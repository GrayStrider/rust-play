//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
#![allow(dead_code)]
#![allow(unused_variables)]
#[allow(unused_imports)]

#[macro_use]
extern crate pipeline;

mod basics;

mod primitives;
mod control_flow;

mod mutability_variables;
mod structs;
mod enums;
mod patterns;

mod ownership;
mod fp;
mod option;
mod std;
mod traits;
