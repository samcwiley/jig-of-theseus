use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    ir::{Duration, Embellishment, Measure, Note, Part, Pitch, TimeSignature, Tune},
    writers::MusicWriter,
};

pub struct LilyWriter {
    pub writer: BufWriter<File>,
}

impl MusicWriter for LilyWriter {
    fn write_note(&mut self, note: &Note) -> std::io::Result<()> {
        let Note {
            pitch,
            duration,
            embellishment,
        } = note;
        if let Some(embellishment) = embellishment {
            write!(
                self.writer,
                "{} {}{} ",
                get_lily_embellishment(embellishment),
                get_lily_pitch(pitch),
                get_lily_duration(duration)
            )?;
        } else {
            write!(
                self.writer,
                "{}{} ",
                get_lily_pitch(pitch),
                get_lily_duration(duration)
            )?;
        }
        Ok(())
    }

    fn write_measure(&mut self, measure: &Measure, _measure_number: usize) -> std::io::Result<()> {
        write!(self.writer, "\t\t")?;
        for note in &measure.notes {
            self.write_note(note)?;
        }
        writeln!(self.writer, "|")?;
        Ok(())
    }

    fn write_part(
        &mut self,
        part: &Part,
        _part_number: usize,
        _time_signature: TimeSignature,
    ) -> std::io::Result<()> {
        writeln!(self.writer, "\t\\repeat volta 2 {{")?;
        for (measure_number, measure) in part.bars.iter().enumerate() {
            self.write_measure(measure, measure_number)?;
        }
        writeln!(self.writer, "\t}}")?;
        Ok(())
    }

    fn write_tune(&mut self, tune: &Tune) -> std::io::Result<()> {
        let internal_name = tune.name.to_ascii_lowercase().replace(' ', "_");
        writeln!(self.writer, "{internal_name} = {{")?;
        writeln!(self.writer, "\t\\time 6/8")?;
        for (part_number, part) in tune.parts.iter().enumerate() {
            self.write_part(part, part_number, tune.time_signature)?;
        }
        writeln!(self.writer, "}}")?;
        Ok(())
    }
}

pub fn write_lily_file(writer: &mut LilyWriter, tune: &Tune) -> std::io::Result<()> {
    let pre_tune_junk = r#"\version "2.25.21"

\include "bagpipe.ly" 

\include "./music/includes/scw_bagpipe.ly"
\include "./music/includes/score_settings.ly"

filename = "atholl_highlanders.ly"
source = "trad, simplified"

#(allow-volta-hook "|")


voltaTwo = \markup  { \hspace #20 \italic \fontsize #+5 { "2" }  }
    
    "#;
    write!(writer.writer, "{pre_tune_junk}")?;
    writer.write_tune(tune)?;

    let post_tune_junk = r#"
    \header { 
          title = \markup  \override #'(line-width . 82) 
          { 
            \column {  
              \center-align {
                \line { Atholl Highlanders
                }
              }
            }
          }
                  
          subtitle = ""
          composer = "trad, simplified"
          arranger = ""
          meter = "" 
         }    




\paper {
	#(set-paper-size "letter" 'portrait)
}



\score {
	\new GrandStaff <<
		\new Staff = "GHB" <<
			\new Voice {
			        \global
				\atholl_highlanders
			}
		>>		
	>>
        \layout { \ScoreLayout 
                  \context { 
                             \Score
                             \override SpacingSpanner.base-shortest-duration = #(ly:make-moment 1/2) 
                           }            
                }
          
                \header{
        }
}
    "#;
    write!(writer.writer, "{post_tune_junk}")?;
    Ok(())
}

fn get_lily_pitch(pitch: &Pitch) -> &'static str {
    match pitch {
        Pitch::LowG => "G",
        Pitch::LowA => "a",
        Pitch::B => "b",
        Pitch::C => "c",
        Pitch::D => "d",
        Pitch::E => "e",
        Pitch::F => "f",
        Pitch::HighG => "g",
        Pitch::HighA => "A",
    }
}

fn get_lily_duration(duration: &Duration) -> &'static str {
    match duration {
        Duration::ThirtySecond => "32",
        Duration::Sixteenth => "16",
        Duration::Eighth => "8",
        Duration::Quarter => "4",
        Duration::Half => "2",
        Duration::Whole => "1",
        Duration::DottedSixteenth => "16.",
        Duration::DottedEighth => "8.",
        Duration::DottedQuarter => "4.",
        Duration::DottedHalf => "2.",
        Duration::DottedWhole => "1.",
    }
}

fn get_lily_embellishment(embellishment: &Embellishment) -> String {
    match embellishment {
        Embellishment::GraceNote(pitch) => format!("\\gr{}", get_lily_pitch(pitch)),
        Embellishment::Doubling(pitch) => format!("\\dbl{}", get_lily_pitch(pitch)),
        Embellishment::HalfDoubling(pitch) => format!("\\hdbl{}", get_lily_pitch(pitch)),
        Embellishment::ThumbDoubling(pitch) => format!("\\tdbl{}", get_lily_pitch(pitch)),
        Embellishment::Slur(pitch) => format!("\\slur{}", get_lily_pitch(pitch)),
        Embellishment::HornpipeShake(pitch) => format!("\\shake{}", get_lily_pitch(pitch)),
        Embellishment::Grip => String::from("\\grip"),
        Embellishment::BGrip => String::from("\\bgrip"),
        Embellishment::Taorluath => String::from("\\taor"),
        Embellishment::BTaorluath => String::from("\\btaor"),
        Embellishment::LGTaorluath => String::from("\\Gtaor"),
        Embellishment::ThrowD => String::from("thrwd"),
        Embellishment::Crunluath => String::from("\\crun"),
        Embellishment::BCrunluath => String::from("\\dcrun"),
        Embellishment::LGCrunluath => String::from("\\Gcrun"),
        Embellishment::HeavyCrunluath => String::from("\\pgrace { G32[ d G e G f G }"),
        Embellishment::HeavyBCrunluath => String::from("\\pgrace { G32[ b G e G f G] }"),
        // took me a minute to realize these had the same grace notes
        Embellishment::Edre | Embellishment::Endari => String::from("\\dre"),
        Embellishment::Dare => String::from("\\dare"),
        Embellishment::Chedari => String::from("\\dari"),
        Embellishment::Embari => String::from("\\bari"),
        Embellishment::Birl => String::from("\\wbirl"),
        Embellishment::Abirl => String::from("\\birl"),
        Embellishment::Gbirl => String::from("\\gbirl"),
        Embellishment::Darodo => String::from("\\darodo"),
        Embellishment::Hodro => String::from("\\catchc"),
        Embellishment::Hiotro => String::from("\\catchb"),
        Embellishment::Tie(_) => String::from("~"),
    }
}

// pub fn write_lily_note(note: &Note) -> String {}

// have a trait called music writer
// it has methods "write_note", "write_measure", etc.
// implement those traits for lilypond, musescore, etc.
//
// have the lilypond writer struct hold the bufwriter itself
// then you can use write!() to write things
