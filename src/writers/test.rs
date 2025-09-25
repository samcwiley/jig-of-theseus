#[cfg(test)]
use crate::{
    lilypond::process_bar,
    writers::bmw::{self, BeamSide},
    writers::lilypond::{self},
};

/// Test BMW Beaming
#[test]
fn test_bmw_beams() {
    let bar = process_bar(r"\grg c8 [e8 \gra e8] \grg d8 [e8 \gra e8]");
    let beats = bar.get_beats();
    assert_eq!(beats.len(), 2);
    let beams1 = bmw::get_beams(&beats[0]);
    let beams2 = bmw::get_beams(&beats[1]);
    assert_eq!(
        beams1,
        vec![
            Some(BeamSide::Right),
            Some(BeamSide::Left),
            Some(BeamSide::Left)
        ]
    );
    assert_eq!(
        beams2,
        vec![
            Some(BeamSide::Right),
            Some(BeamSide::Left),
            Some(BeamSide::Left)
        ]
    );

    let bar = process_bar(r"\grg a4 \grd c8 \grg b4 \grd c8");
    let beats = bar.get_beats();
    assert_eq!(beats.len(), 2);
    let beams1 = bmw::get_beams(&beats[0]);
    let beams2 = bmw::get_beams(&beats[1]);
    assert_eq!(beams1, vec![None, None]);
    assert_eq!(beams2, vec![None, None]);
}
/// More lilypond beaming
#[test]
fn test_lily_beams() {
    let bar = process_bar(r"\grg c8 [e8 \gra e8] \grg d8 [e8 \gra e8]");
    let beats = bar.get_beats();
    let beams1 = lilypond::get_beams(&beats[0]);
    let beams2 = lilypond::get_beams(&beats[1]);
    assert_eq!(beams1, vec![Some((1, 2))]);
    assert_eq!(beams2, vec![Some((1, 2))]);
    let beats = process_bar(r"\grg a4 \grd c8 \grg b4 \grd c8").get_beats();
    let beams1 = lilypond::get_beams(&beats[0]);
    let beams2 = lilypond::get_beams(&beats[1]);
    assert_eq!(beams1, Vec::new());
    assert_eq!(beams2, Vec::new());
}
