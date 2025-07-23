#![allow(unused_imports)]
use crate::parsers::{ir::*, lilypond::*};

fn main() -> Result<(), std::io::Error> {
    process_lily()?;
    Ok(())
}

mod parsers;
pub use crate::parsers::*;
