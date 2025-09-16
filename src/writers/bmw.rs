use std::{
    fmt,
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    ir::{Duration, Embellishment, Measure, Note, Part, Pitch, Tune},
    writers::MusicWriter,
};

/// Wraps `BufWriter` with functions for writing `.bww` files
pub struct BMWWriter {
    pub writer: BufWriter<File>,
}

/// Enum for handling beam directions for beamed 8th notes, 16ths, etc.
enum BeamSide {
    Left,
    Right,
}

impl BeamSide {
    /// For quickly creating beams
    fn from(side: char) -> Self {
        match side {
            'l' => Self::Left,
            'r' => Self::Right,
            _ => panic!("Invalid beam side"),
        }
    }
}

impl fmt::Display for BeamSide {
    /// Writes beams for use in beamed 8th notes, etc.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = match self {
            Self::Left => 'l',
            Self::Right => 'r',
        };
        write!(f, "{side}")
    }
}

impl BMWWriter {
    /// Handles writing a note with beaming or calls off to a standard, non-beamed note writer
    fn write_bmw_note(&mut self, note: &Note, beam_side: Option<BeamSide>) -> std::io::Result<()> {
        if let Some(beam_side) = beam_side {
            let Note {
                pitch,
                duration,
                embellishment,
            } = note;
            let embellishment = embellishment.as_ref().expect("couldn't read embellishment");
            let bmw_pitch = get_bmw_pitch(pitch);
            let bmw_duration = get_bmw_duration(duration, pitch);
            write!(
                self.writer,
                "{embellishment} {bmw_pitch}{beam_side}_{bmw_duration}"
            )?;
            Ok(())
        } else {
            self.write_note(note)
        }
    }
}

impl MusicWriter for BMWWriter {
    /// Writes note without beaming
    fn write_note(&mut self, note: &Note) -> std::io::Result<()> {
        let Note {
            pitch,
            duration,
            embellishment,
        } = note;
        let embellishment = embellishment.as_ref().expect("couldn't read embellishment");
        let bmw_pitch = get_bmw_pitch(pitch);
        let bmw_duration = get_bmw_duration(duration, pitch);
        write!(self.writer, "{embellishment} {bmw_pitch}_{bmw_duration}")?;
        Ok(())
    }

    /// Writes out a full measure of notes; handles beaming logic
    fn write_measure(&mut self, measure: &Measure) -> std::io::Result<()> {
        todo!()
    }

    /// Writes a series of measures; handles barline logic
    fn write_part(&mut self, part: &Part) -> std::io::Result<()> {
        todo!()
    }

    /// Writes a series of parts along with requisite metadata used in BMW files
    fn write_tune(&mut self, tune: &Tune) -> std::io::Result<()> {
        todo!()
    }
}

/// For getting display pitches used in notes
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

/// For use in dots and embellishments
struct BMWLowercase {
    pitch: Pitch,
}

impl BMWLowercase {
    fn new(pitch: &Pitch) -> Self {
        Self {
            pitch: pitch.clone(),
        }
    }
}

/// for writing pitches in dots and embellishments
impl fmt::Display for BMWLowercase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_pitch = match self.pitch {
            Pitch::LowG => "lg",
            Pitch::LowA => "la",
            Pitch::B => "b",
            Pitch::C => "c",
            Pitch::D => "d",
            Pitch::E => "e",
            Pitch::F => "f",
            Pitch::HighG => "hg",
            Pitch::HighA => "ha",
        };
        write!(f, "{lowercase_pitch}")
    }
}

/// Used for writing dots in BMW files
struct Dot {
    pitch: Pitch,
}

impl Dot {
    /// Creates a dot on the given pitch
    fn new(pitch: &Pitch) -> Self {
        Self {
            pitch: pitch.clone(),
        }
    }
}

impl fmt::Display for Dot {
    /// Prepends a `'` symbol to the lowercase
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}", BMWLowercase::new(&self.pitch))
    }
}

/// For handling durations in BMW
struct BMWDuration {
    pub stem_value: u8,
    pub dot: Option<Dot>,
}

impl fmt::Display for BMWDuration {
    /// Writes a stemvalue, space, and dot on a the appropriate note
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let BMWDuration { stem_value, dot } = self;
        if let Some(dot) = dot {
            write!(f, "{stem_value} {dot}")
        } else {
            write!(f, "{stem_value}")
        }
    }
}

/// Takes in duration and pitch and returns a [`BMWDuration`] object containing a
/// stem value (2, 4, 8, etc) and an optional dot on the provided pitch
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

/// For writing out embellishments in BMW. The embellishments marked `todo!()` I
/// could not find examples of in tunes
fn get_bmw_embellishment(embellishment: &Embellishment) -> String {
    match embellishment {
        Embellishment::GraceNote(pitch) => match pitch {
            Pitch::B => todo!(),
            Pitch::LowG | Pitch::LowA | Pitch::C | Pitch::F => {
                format!("str{}", BMWLowercase::new(pitch))
            }
            Pitch::D | Pitch::E | Pitch::HighG => format!("{}g", BMWLowercase::new(pitch)),
            Pitch::HighA => String::from("tg"),
        },
        Embellishment::Doubling(pitch) => format!("db{}", BMWLowercase::new(pitch)),
        Embellishment::HalfDoubling(pitch) => format!("hdb{}", BMWLowercase::new(pitch)),
        Embellishment::ThumbDoubling(pitch) => format!("tdb{}", BMWLowercase::new(pitch)),
        Embellishment::Slur(pitch) => {
            match pitch {
                Pitch::LowG => unimplemented!("This shouldn't be possible"),
                Pitch::LowA => todo!(),
                Pitch::B => String::from("gstb"),
                Pitch::C => todo!(),
                // todo!() light d strike
                Pitch::D => String::from("lgstd"),
                Pitch::E => todo!(),
                Pitch::F => todo!(),
                Pitch::HighG => todo!(),
                Pitch::HighA => String::from("dblha"),
            }
        }
        Embellishment::HornpipeShake(pitch) => format!("pel{}", BMWLowercase::new(pitch)),
        Embellishment::Grip => String::from("grp"),
        Embellishment::BGrip => String::from("grpb"),
        Embellishment::Taorluath => String::from("taor"),
        Embellishment::BTaorluath => String::from("taorb"),
        Embellishment::LGTaorluath => todo!(),
        Embellishment::ThrowD => String::from("thrd"),
        Embellishment::Crunluath => String::from("crunl"),
        Embellishment::BCrunluath => String::from("crunlb"),
        Embellishment::LGCrunluath => todo!(),
        Embellishment::HeavyCrunluath => todo!(),
        Embellishment::HeavyBCrunluath => todo!(),
        Embellishment::Edre => String::from("edre"),
        Embellishment::Dare => String::from("dare"),
        Embellishment::Chedari => todo!(),
        Embellishment::Embari => todo!(),
        Embellishment::Endari => todo!(),
        Embellishment::Birl => String::from("brl"),
        Embellishment::Abirl => String::from("abr"),
        Embellishment::Gbirl => String::from("gbr"),
        Embellishment::Darodo => String::from("bubly"),
        Embellishment::Hodro => String::from("ggrpc"),
        Embellishment::Hiotro => String::from("ggrpb"),
        Embellishment::Tie(pitch) => format!("^t{}", BMWLowercase::new(pitch)),
    }
}
