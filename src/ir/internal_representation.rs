#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
//use crate::utils::math::assert_fp_eq;
use std::fmt;

use crate::utils::math::f32_eq;

#[derive(Debug, Clone, Copy)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Duration,
    pub embellishment: Option<Embellishment>,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.embellishment {
            None => write!(f, "{} on {}", self.duration, self.pitch),
            Some(embellishment) => {
                write!(
                    f,
                    "{} on {} with {}",
                    self.duration, self.pitch, embellishment
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pitch {
    LowG,
    LowA,
    B,
    C,
    D,
    E,
    F,
    HighG,
    HighA,
}

impl Pitch {
    /// Converts lilypond note character to a Pitch object
    ///
    /// # Panics
    ///
    /// Panics if a provided lilypond note is invalid
    #[must_use]
    pub fn from_lily_char(pitch: char) -> Self {
        match pitch {
            'G' => Self::LowG,
            'a' => Self::LowA,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            'E' => Self::E,
            'F' => Self::F,
            'g' => Self::HighG,
            'A' => Self::HighA,
            _ => panic!("invalid lily note character \"{pitch}\""),
        }
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pitch = match self {
            Pitch::LowG => "Low G",
            Pitch::LowA => "Low A",
            Pitch::B => "B",
            Pitch::C => "C",
            Pitch::D => "D",
            Pitch::E => "E",
            Pitch::F => "F",
            Pitch::HighG => "High G",
            Pitch::HighA => "High A",
        };
        write!(f, "{pitch}")
    }
}

#[derive(Clone, Debug, Copy)]
pub enum Duration {
    ThirtySecond,
    Sixteenth,
    Eighth,
    Quarter,
    Half,
    Whole,
    DottedSixteenth,
    DottedEighth,
    DottedQuarter,
    DottedHalf,
    DottedWhole,
}

impl Duration {
    #[must_use]
    pub fn eighths(&self) -> f32 {
        match self {
            Duration::ThirtySecond => 0.25,
            Duration::Sixteenth => 0.5,
            Duration::Eighth => 1.0,
            Duration::Quarter => 2.0,
            Duration::Half => 4.0,
            Duration::Whole => 8.0,
            Duration::DottedSixteenth => 0.75,
            Duration::DottedEighth => 1.5,
            Duration::DottedQuarter => 3.0,
            Duration::DottedHalf => 6.0,
            Duration::DottedWhole => 12.0,
        }
    }
    #[must_use]
    pub fn is_beamed(&self) -> bool {
        match self {
            Duration::Quarter
            | Duration::Half
            | Duration::Whole
            | Duration::DottedQuarter
            | Duration::DottedHalf
            | Duration::DottedWhole => false,
            Duration::ThirtySecond
            | Duration::Sixteenth
            | Duration::Eighth
            | Duration::DottedSixteenth
            | Duration::DottedEighth => true,
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let duration = match self {
            Duration::ThirtySecond => "Thirty-Second Note",
            Duration::Sixteenth => "Sixteenth Note",
            Duration::Eighth => "Eighth Note",
            Duration::Quarter => "Quarter Note",
            Duration::Half => "Half Note",
            Duration::Whole => "Whole Note",
            Duration::DottedSixteenth => "Dotted Sixteenth Note",
            Duration::DottedEighth => "Dotted Eighth Note",
            Duration::DottedQuarter => "Dotted Quarter Note",
            Duration::DottedHalf => "Dotted Half Note",
            Duration::DottedWhole => "Dotted Whole Note",
        };
        write!(f, "{duration}")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Embellishment {
    GraceNote(Pitch),
    Doubling(Pitch),
    HalfDoubling(Pitch),
    ThumbDoubling(Pitch),
    Slur(Pitch),
    HornpipeShake(Pitch),
    Grip,
    BGrip,
    Taorluath,
    BTaorluath,
    LGTaorluath,
    ThrowD,
    Crunluath,
    BCrunluath,
    LGCrunluath,
    HeavyCrunluath,
    HeavyBCrunluath,
    Edre,
    Dare,
    Chedari,
    Embari,
    Endari,
    Birl,
    Abirl,
    Gbirl,
    Darodo,
    Hodro,
    Hiotro,
    Tie(Pitch),
}

impl Embellishment {
    #[must_use]
    pub fn new() -> Self {
        Self::GraceNote(Pitch::HighG)
    }
}
impl Default for Embellishment {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Embellishment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let embellishment = match self {
            Embellishment::GraceNote(pitch) => &format!("{pitch} Grace Note"),
            Embellishment::Doubling(pitch) => &format!("{pitch} Doubling"),
            Embellishment::Taorluath => "Taorluath",
            Embellishment::LGTaorluath => "Low G Taorluath",
            Embellishment::ThrowD => "D Throw",
            Embellishment::Crunluath => "Crunluath",
            Embellishment::HalfDoubling(pitch) => &format!("{pitch} Half Doubling"),
            Embellishment::Grip => "Grip",
            Embellishment::BGrip => "Grip with B grace note",
            Embellishment::Edre => "Edre",
            Embellishment::Dare => "Dare",
            Embellishment::Chedari => "Chedari",
            Embellishment::Embari => "Embari",
            Embellishment::Endari => "Endari",
            Embellishment::Birl | Embellishment::Abirl => "Birl",
            Embellishment::Gbirl => "G grace note birl",
            Embellishment::Darodo => "Darodo",
            Embellishment::Slur(pitch) => &format!("{pitch} Slur"),
            Embellishment::ThumbDoubling(pitch) => &format!("{pitch} Thumb Doubling"),
            Embellishment::HornpipeShake(pitch) => &format!("{pitch} Hornpipe Shake"),
            Embellishment::BTaorluath => "Taorluath with B Grace Note",
            Embellishment::BCrunluath => "Crunluath with B Grace Note",
            Embellishment::LGCrunluath => "Low G Crunluath",
            Embellishment::HeavyBCrunluath => "Heavy Crunluath with B Grace Note",
            Embellishment::Hodro => "Hodro",
            Embellishment::Hiotro => "Hiotro",
            Embellishment::HeavyCrunluath => "Heavy Crunluath",
            Embellishment::Tie(_) => "Tie",
        };
        write!(f, "{embellishment}")
    }
}

#[derive(Debug, Clone)]
pub struct Measure {
    pub notes: Vec<Note>,
    pub time_signature: TimeSignature,
}

impl fmt::Display for Measure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for note in &self.notes {
            write!(f, "{note}, ")?;
        }
        Ok(())
    }
}

pub struct Part {
    pub bars: Vec<Measure>,
}
impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, bar) in self.bars.iter().enumerate() {
            writeln!(f, "Measure {i}: {bar}")?;
        }
        Ok(())
    }
}

pub enum TuneType {
    March,
    Jig,
}
impl fmt::Display for TuneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tune_type = match self {
            TuneType::March => "March",
            TuneType::Jig => "Jig",
        };
        write!(f, "{tune_type}")
    }
}

pub struct Tune {
    pub name: String,
    pub parts: Vec<Part>,
    pub time_signature: TimeSignature,
    pub tune_type: TuneType,
    pub composer: String,
}

impl fmt::Display for Tune {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tune: {}", self.name)?;
        for (i, part) in self.parts.iter().enumerate() {
            writeln!(f, "Part {i}")?;
            writeln!(f, "{part}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TimeSignature {
    SixEight,
}

impl TimeSignature {
    fn eights_per_beat(self) -> usize {
        match self {
            TimeSignature::SixEight => 3,
        }
    }
    fn beats_per_bar(self) -> usize {
        match self {
            TimeSignature::SixEight => 2,
        }
    }
    fn eights_per_bar(self) -> usize {
        self.eights_per_beat() * self.beats_per_bar()
    }
}

impl fmt::Display for TimeSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sig = match self {
            Self::SixEight => "6/8",
        };
        write!(f, "{sig}")
    }
}

pub struct Beat {
    pub notes: Vec<Note>,
}

impl Beat {
    #[must_use]
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }
    #[must_use]
    pub fn eighths(&self) -> f32 {
        self.notes.iter().map(|n| n.duration.eighths()).sum::<f32>()
    }
    pub fn push(&mut self, note: &Note) {
        self.notes.push(*note);
    }
}

impl Default for Beat {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Beat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for note in &self.notes {
            write!(f, "{note} ")?;
        }
        Ok(())
    }
}

impl Measure {
    /// For checking if a measure has the correct amount of beats
    #[must_use]
    pub fn validate(&self) -> bool {
        f32_eq(
            self.notes.iter().map(|n| n.duration.eighths()).sum::<f32>(),
            self.time_signature.eights_per_bar() as f32,
        )
    }

    /// For examining the notes within a measure and figuring how the notes
    /// should be beamed todo: add in logic for ties
    /// # Panics
    ///
    /// Will panic if the bar has an improper amount of beats, todo: probably
    /// remove this
    #[must_use]
    pub fn get_beats(&self) -> Vec<Beat> {
        assert!(self.validate(), "Invalid number of beats in bar");
        let total_eighths = self.time_signature.eights_per_bar() as f32;
        let total_beats = self.time_signature.beats_per_bar();
        let eighths_per_beat = self.time_signature.eights_per_beat() as f32;
        let mut beats = Vec::with_capacity(total_beats);
        let mut current_beat = Beat::new();

        for note in &self.notes {
            if note.duration.eighths() <= eighths_per_beat - current_beat.eighths() {
                current_beat.notes.push(*note);
            } else {
                beats.push(current_beat);
                current_beat = Beat::new();
                current_beat.notes.push(*note);
            }
            if f32_eq(current_beat.eighths(), eighths_per_beat) {
                beats.push(current_beat);
                current_beat = Beat::new();
            }
        }
        assert!(
            f32_eq(beats.iter().map(Beat::eighths).sum::<f32>(), total_eighths),
            "beats don't add up at end"
        );
        beats
    }
}
