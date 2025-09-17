#[cfg(test)]
use crate::{
    lilypond::process_bar,
    writers::bmw::{BeamSide, get_beams},
};
#[test]
fn test_beams() {
    let bar = process_bar(r"\grg c8 [e8 \gra e8] \grg d8 [e8 \gra e8]");
    let beats = bar.get_beats();
    assert_eq!(beats.len(), 2);
    let beams1 = get_beams(&beats[0]);
    let beams2 = get_beams(&beats[1]);
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
}

#[test]
fn test_beams_2() {
    let bar = process_bar(r"\grg a4 \grd c8 \grg b4 \grd c8");
    let beats = bar.get_beats();
    assert_eq!(beats.len(), 2);
    let beams1 = get_beams(&beats[0]);
    let beams2 = get_beams(&beats[1]);
    assert_eq!(beams1, vec![None, None]);
    assert_eq!(beams2, vec![None, None]);
}
