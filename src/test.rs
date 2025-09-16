#[cfg(test)]
use crate::parsers::lilypond::process_bar;

#[test]
fn test_beats() {
    let bar = process_bar(r"\grg c8 [e8 \gra e8] \grg d8 [e8 \gra e8]");
    assert_eq!(bar.notes.len(), 6);
    let beats = bar.get_beats();
    for beat in &beats {
        println!("{beat}");
    }
    assert_eq!(beats.len(), 2);
}

#[test]
fn test_beats_2() {
    let bar = process_bar(r"\grg a4 \grd c8 \grg b4 \grd c8");
    assert_eq!(bar.notes.len(), 4);
    let beats = bar.get_beats();
    for beat in &beats {
        println!("{beat}");
    }
    assert_eq!(beats.len(), 2);
}

#[test]
fn test_beats_3() {
    let bar = process_bar(r"\grg a4. A4.");
    assert_eq!(bar.notes.len(), 2);
    let beats = bar.get_beats();
    for beat in &beats {
        println!("{beat}");
    }
    assert_eq!(beats.len(), 2);
}
