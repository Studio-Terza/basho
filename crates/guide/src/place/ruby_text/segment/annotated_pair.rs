use std::str::FromStr;

use super::Segment;
use super::script::Script;
use crate::place::ruby_text::ruby_error::RubyError;
use crate::words::{Spelling, Written};

pub struct AnnotatedPair([Segment; AnnotatedPair::LENGTH]);

impl AnnotatedPair {
    const LENGTH: usize = 2;
}

impl FromStr for AnnotatedPair {
    type Err = RubyError;

    fn from_str(piece: &str) -> Result<AnnotatedPair, RubyError> {
        let (annotation, rest) = piece
            .split_once('}')
            .ok_or_else(|| RubyError::UnclosedRuby(Written::new(piece.to_owned())))?;
        let (base, reading) = annotation
            .split_once('|')
            .ok_or_else(|| RubyError::MissingReading(Written::new(annotation.to_owned())))?;
        if reading
            .chars()
            .map(Script::from)
            .all(|script| script == Script::Kana)
        {
            Ok(AnnotatedPair([
                Segment::Ruby {
                    base: Spelling::new(base.to_owned()),
                    reading: Spelling::new(reading.to_owned()),
                },
                Segment::try_from(rest)?,
            ]))
        } else {
            Err(RubyError::ReadingNotKana(Written::new(reading.to_owned())))
        }
    }
}

impl From<AnnotatedPair> for Vec<Segment> {
    fn from(pair: AnnotatedPair) -> Vec<Segment> {
        Vec::from(pair.0)
    }
}
