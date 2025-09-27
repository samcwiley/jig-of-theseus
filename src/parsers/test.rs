#[cfg(test)]
use crate::{ir::internal_representation::Pitch, lilypond::process_bar};

#[test]
fn test_lily_notes() {
    let bar = process_bar(r"\grg G32 [a32 b16] \grg c32 [d32 e16] \grg f32 [g32 A16] |");
    assert_eq!(bar.notes.len(), 9);
    let pitches = bar.notes.iter().map(|note| note.pitch).collect::<Vec<_>>();
    assert_eq!(
        pitches,
        vec![
            Pitch::LowG,
            Pitch::LowA,
            Pitch::B,
            Pitch::C,
            Pitch::D,
            Pitch::E,
            Pitch::F,
            Pitch::HighG,
            Pitch::HighA
        ]
    );
}
