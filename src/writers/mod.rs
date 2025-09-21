use crate::ir::{Measure, Note, Part, TimeSignature, Tune};

pub mod bmw;
pub mod lilypond;
pub mod test;
pub trait MusicWriter {
    fn write_note(&mut self, note: &Note) -> std::io::Result<()>;

    fn write_measure(&mut self, measure: &Measure, measure_number: usize) -> std::io::Result<()>;

    fn write_part(
        &mut self,
        part: &Part,
        part_number: usize,
        time_signature: TimeSignature,
    ) -> std::io::Result<()>;

    fn write_tune(&mut self, tune: &Tune) -> std::io::Result<()>;
}
