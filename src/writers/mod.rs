use crate::ir::{Measure, Note, Part, Tune};

pub mod lilypond;

pub trait MusicWriter {
    fn write_note(&mut self, note: &Note) -> std::io::Result<()>;

    fn write_measure(&mut self, measure: &Measure) -> std::io::Result<()>;

    fn write_part(&mut self, part: &Part) -> std::io::Result<()>;

    fn write_tune(&mut self, tune: &Tune) -> std::io::Result<()>;
}
