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
    Eighth,
    Quarter,
    DottedQuarter,
}

impl Duration {
    pub fn eighths(&self) -> usize {
        match self {
            Duration::Eighth => 1,
            Duration::Quarter => 2,
            Duration::DottedQuarter => 3,
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let duration = match self {
            Duration::Eighth => "Eighth Note",
            Duration::Quarter => "Quarter Note",
            Duration::DottedQuarter => "Dotted Quarter Note",
        };
        write!(f, "{duration}")
    }
}

pub enum Embellishment {
    GrG,
    GrE,
    GrD,
}

impl fmt::Display for Embellishment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let embellishment = match self {
            Embellishment::GrD => "D Grace Note",
            Embellishment::GrE => "E Grace Note",
            Embellishment::GrG => "G Grace Note",
        };
        write!(f, "{embellishment}")
    }
}

pub struct Measure {
    pub notes: Vec<Note>,
}

impl Measure {
    pub fn validate(&self) -> bool {
        self.notes
            .iter()
            .map(|n| n.duration.eighths())
            .sum::<usize>()
            == 6
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
