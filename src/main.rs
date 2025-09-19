#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
use std::{fs::File, io::BufWriter};

use crate::{
    parsers::lilypond::process_lily,
    writers::{
        bmw::{BMWWriter, write_bmw_file},
        lilypond::{LilyWriter, write_lily_file},
    },
};

fn main() -> Result<(), std::io::Error> {
    let tune = process_lily()?;
    let lily_out = File::create("atholl_highlanders_out.ly")?;
    let bmw_out = File::create("atholl_highlanders_out.bww")?;

    let lily_buf_writer = BufWriter::new(lily_out);
    let bmw_buf_writer = BufWriter::new(bmw_out);

    let mut lily_writer = LilyWriter {
        writer: lily_buf_writer,
    };
    let mut bmw_writer = BMWWriter {
        writer: bmw_buf_writer,
    };

    write_lily_file(&mut lily_writer, &tune)?;
    write_bmw_file(&mut bmw_writer, &tune)?;
    Ok(())
}

mod parsers;
mod writers;
pub use crate::parsers::*;
mod test;
mod utils;
