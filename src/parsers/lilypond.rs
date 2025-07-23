use crate::ir::{Duration, Embellishment, Measure, Note, Pitch, Tune};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq)]
pub enum Group {
    Note,
    Embellishment,
}

// pub fn make_lily_note(embellishment: Option<Embellishment>)

pub fn process_lily() -> Result<(), std::io::Error> {
    let f = File::open("music/atholl_highlanders.ly")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();

    while let Some(meta) = lines.next() {
        if meta? == "atholl_highlanders = {" {
            break;
        }
    }

    let bar_regex =
        Regex::new(r"^(?:\s*\[?\s*(?:\\[a-zA-Z]+|[GabcdefgA]\d\.?)\s*\]?\s*)+\|$").unwrap();

    for line in lines {
        let line = line?;
        if bar_regex.is_match(&line) {
            let bar = process_bar(&line);
            println!("{bar}");
        }
    }

    Ok(())
}

fn process_bar(line: &str) -> Measure {
    let mut tokenized_bar = Vec::new();
    let extract_regex =
        Regex::new(r"(?<embellishment>\\[a-zA-Z]+)|(?<note>[GabcdefgA]\d\.?)").unwrap();
    let captures = extract_regex.captures_iter(&line);
    for capture in captures {
        if let Some(m) = capture.name("embellishment") {
            tokenized_bar.push((Group::Embellishment, m.as_str().to_string()));
        } else if let Some(m) = capture.name("note") {
            tokenized_bar.push((Group::Note, m.as_str().to_string()))
        }
    }
    let mut current_embellishment = None;
    let mut notes = Vec::new();
    for token in tokenized_bar {
        if token.0 == Group::Embellishment {
            current_embellishment = Some(process_lily_embellishment(&token.1));
        } else if token.0 == Group::Note {
            notes.push(process_lily_note(&token.1, current_embellishment));
            current_embellishment = None;
        }
    }
    Measure { notes }
}

fn process_lily_embellishment(embellishment: &str) -> Embellishment {
    match embellishment {
        r"\grg" => Embellishment::GrG,
        r"\grd" => Embellishment::GrD,
        r"\gre" => Embellishment::GrE,
        _ => panic!("Invalid embellishment {embellishment}"),
    }
}

fn process_lily_note(note: &str, embellishment: Option<Embellishment>) -> Note {
    let pitch = match note.as_bytes()[0] {
        b'G' => Pitch::LowG,
        b'a' => Pitch::LowA,
        b'b' => Pitch::B,
        b'c' => Pitch::C,
        b'd' => Pitch::D,
        b'e' => Pitch::E,
        b'f' => Pitch::F,
        b'g' => Pitch::HighG,
        b'A' => Pitch::HighA,
        _ => panic!("Invalid Pitch in {note}"),
    };
    let duration = match &note.as_bytes()[1..] {
        b"8" => Duration::Eighth,
        b"4" => Duration::Quarter,
        b"4." => Duration::DottedQuarter,
        _ => panic!("Invalid Duration in {note}"),
    };
    Note {
        pitch,
        duration,
        embellishment,
    }
}
