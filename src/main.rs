#![warn(clippy::pedantic)]
use std::{fs::File, io::BufWriter};

use crate::{
    parsers::lilypond::process_lily,
    writers::lilypond::{LilyWriter, write_lily_file},
};

fn main() -> Result<(), std::io::Error> {
    let tune = process_lily()?;
    let output = File::create("atholl_highlanders_out.ly")?;
    let buf_writer = BufWriter::new(output);
    let mut writer = LilyWriter { writer: buf_writer };
    write_lily_file(&mut writer, &tune)?;
    Ok(())
}

mod parsers;
mod writers;
pub use crate::parsers::*;
