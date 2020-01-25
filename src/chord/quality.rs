use crate::chord::errors::ChordError;
use regex::{Match, Regex};

const REGEX_QUALITY_MAJOR: &str = r"^(M\s+|M$|(?i)maj|Maj|Major|major)";
const REGEX_QUALITY_MINOR: &str = r"^(m\s+|m$|(?i)min|Min|Minor|minor)";
const REGEX_QUALITY_DIMINISHED: &str = r"(?i)^(diminished)";
const REGEX_QUALITY_AUGMENTED: &str = r"(?i)^(augmented)";
const REGEX_QUALITY_HALF_DIMINISHED: &str = r"(?i)^(half\s*diminished|halfdiminished)";
const REGEX_QUALITY_DOMINANT: &str = r"(?i)^(dominant)";

#[derive(Debug, PartialEq)]
pub enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
    HalfDiminished,
    Dominant,
}

impl Quality {
    pub fn from_regex(string: &str) -> Result<(Self, Option<Match>), ChordError> {
        use Quality::*;
        let regexes = vec![
            (Regex::new(REGEX_QUALITY_MAJOR), Major),
            (Regex::new(REGEX_QUALITY_MINOR), Minor),
            (Regex::new(REGEX_QUALITY_DIMINISHED), Diminished),
            (Regex::new(REGEX_QUALITY_AUGMENTED), Augmented),
            (Regex::new(REGEX_QUALITY_HALF_DIMINISHED), HalfDiminished),
            (Regex::new(REGEX_QUALITY_DOMINANT), Dominant),
        ];

        for (regex, quality_enum) in regexes {
            let mode = regex?.find(string.trim());

            match mode {
                Some(quality_match) => return Ok((quality_enum, Some(quality_match))),
                _ => {}
            };
        }

        Ok((Major, None))
    }
}
