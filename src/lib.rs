#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]

pub mod parsers;
pub mod writers;
pub use crate::parsers::*;
pub mod ir;
mod utils;
