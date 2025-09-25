use crate::ir::internal_representation::{Measure, Note, Part, TimeSignature, Tune};

pub mod bmw;
pub mod lilypond;
pub mod test;
pub trait MusicWriter {
    /// Function for writing a note to an output file
    ///
    /// # Errors
    ///
    /// This function returns Results from the internal `write!` macro
    fn write_note(&mut self, note: &Note) -> std::io::Result<()>;

    /// Function for writing a measure of notes to an output file
    ///
    /// # Errors
    ///
    /// This function returns Results from the `write_note` and `write!` macro
    fn write_measure(&mut self, measure: &Measure, measure_number: usize) -> std::io::Result<()>;

    /// Function for writing a Part of Measures to an output file
    ///
    /// # Errors
    ///
    /// This returns Results from the `write_measure` and `write!` macro
    fn write_part(
        &mut self,
        part: &Part,
        part_number: usize,
        time_signature: TimeSignature,
    ) -> std::io::Result<()>;

    /// Function for writing a Tune of Parts to an output file
    ///
    /// # Errors
    ///
    /// This returns Results from the `write_part` and `write!` macro
    fn write_tune(&mut self, tune: &Tune) -> std::io::Result<()>;
}
