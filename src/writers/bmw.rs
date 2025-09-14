use std::{
    fmt,
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    ir::{Duration, Embellishment, Measure, Note, Part, Pitch, Tune},
    writers::MusicWriter,
};

pub struct BMWWriter {
    pub writer: BufWriter<File>,
}

impl MusicWriter for BMWWriter {
    fn write_note(&mut self, note: &Note) -> std::io::Result<()> {
        let Note {
            pitch,
            duration,
            embellishment,
        } = note;
    }

    fn write_measure(&mut self, measure: &Measure) -> std::io::Result<()> {
        todo!()
    }

    fn write_part(&mut self, part: &Part) -> std::io::Result<()> {
        todo!()
    }

    fn write_tune(&mut self, tune: &Tune) -> std::io::Result<()> {
        todo!()
    }
}

fn get_bmw_pitch(pitch: &Pitch) -> &'static str {
    match pitch {
        Pitch::LowG => "LG",
        Pitch::LowA => "LA",
        Pitch::B => "B",
        Pitch::C => "C",
        Pitch::D => "D",
        Pitch::E => "E",
        Pitch::F => "F",
        Pitch::HighG => "HG",
        Pitch::HighA => "HA",
    }
}

struct Dot {
    pitch: Pitch,
}

impl Dot {
    fn new(pitch: &Pitch) -> Self {
        Self {
            pitch: pitch.clone(),
        }
    }
}

impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dot = match self.pitch {
            Pitch::LowG => "'lg",
            Pitch::LowA => "'la",
            Pitch::B => "'b",
            Pitch::C => "'c",
            Pitch::D => "'d",
            Pitch::E => "'e",
            Pitch::F => "'f",
            Pitch::HighG => "'hg",
            Pitch::HighA => "'ha",
        };
        write!(f, "{dot}")
    }
}

struct BMWDuration {
    pub stem_value: u8,
    pub dot: Option<Dot>,
}

impl fmt::Display for BMWDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let BMWDuration { stem_value, dot } = self;
        if let Some(dot) = dot {
            write!(f, "{stem_value} {dot}")
        } else {
            write!(f, "{stem_value}")
        }
    }
}

fn get_bmw_duration(duration: &Duration, pitch: &Pitch) -> BMWDuration {
    let (stem_value, dot) = match duration {
        Duration::ThirtySecond => (32, None),
        Duration::Sixteenth => (16, None),
        Duration::Eighth => (8, None),
        Duration::Quarter => (4, None),
        Duration::Half => (2, None),
        Duration::Whole => (1, None),
        Duration::DottedSixteenth => (16, Some(Dot::new(pitch))),
        Duration::DottedEighth => (8, Some(Dot::new(pitch))),
        Duration::DottedQuarter => (4, Some(Dot::new(pitch))),
        Duration::DottedHalf => (2, Some(Dot::new(pitch))),
        Duration::DottedWhole => (1, Some(Dot::new(pitch))),
    };
    BMWDuration { stem_value, dot }
}
