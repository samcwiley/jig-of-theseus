use std::fmt;

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

#[derive(Debug, PartialEq, Eq, Clone)]
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

pub enum Embellishment {
    GraceNoteHighG,
    GraceNoteE,
    GraceNoteD,
    GraceNoteHighA,
    GraceNoteF,
    GraceNoteC,
    GraceNoteB,
    GraceNoteLowA,
    GraceNoteLowG,
    DoublingLowG,
    DoublingLowA,
    DoublingB,
    DoublingC,
    DoublingD,
    DoublingE,
    DoublingF,
    DoublingHighG,
    DoublingHighA,
}

impl fmt::Display for Embellishment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let embellishment = match self {
            Embellishment::GraceNoteD => "D Grace Note",
            Embellishment::GraceNoteE => "E Grace Note",
            Embellishment::GraceNoteHighG => "G Grace Note",
            Embellishment::GraceNoteB => "B Grace Note",
            Embellishment::GraceNoteC => "C Grace Note",
            Embellishment::GraceNoteLowA => "Low A Grace Note",
            Embellishment::GraceNoteLowG => "Low G Grace Note",
            Embellishment::GraceNoteF => "B Grace Note",
            Embellishment::GraceNoteHighA => "High A Grace Note",
            Embellishment::DoublingLowG => "Low G Doubling",
            Embellishment::DoublingLowA => "Low A Doubling",
            Embellishment::DoublingB => "B Doubling",
            Embellishment::DoublingC => "C Doubling",
            Embellishment::DoublingD => "D Doubling",
            Embellishment::DoublingE => "E Doubling",
            Embellishment::DoublingF => "F Doubling",
            Embellishment::DoublingHighG => "High G Doubling",
            Embellishment::DoublingHighA => "High A Doubling",
        };
        write!(f, "{embellishment}")
    }
}

pub struct Measure {
    pub notes: Vec<Note>,
}

impl Measure {
    pub fn validate(&self) -> bool {
        self.notes.iter().map(|n| n.duration.eighths()).sum::<f32>() as usize == 6
    }
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
    bars: Vec<Measure>,
}
impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, bar) in self.bars.iter().enumerate() {
            writeln!(f, "Measure {i}: {bar}")?;
        }
        Ok(())
    }
}

pub struct Tune {
    name: String,
    parts: Vec<Part>,
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
