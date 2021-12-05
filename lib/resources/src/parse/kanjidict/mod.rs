use std::{io::BufRead, str};

use quick_xml::{
    events::{attributes::Attributes, Event},
    Reader,
};
use types::raw::kanjidict::Character;

use crate::parse::{error::Error, parser::Parse};

/// A kanjidict2 parser
pub struct Parser<R>
where
    R: BufRead,
{
    reader: Reader<R>,
    buf: Vec<u8>,
}

impl<R> Parse<R, Character> for Parser<R>
where
    R: BufRead,
{
    /// Create a new parser
    fn new(r: R) -> Parser<R> {
        Self {
            reader: Reader::from_reader(r),
            buf: Vec::new(),
        }
    }

    /// Parse a kanjidict2 xml file
    fn count(mut self) -> Result<usize, Error> {
        let mut counter = 0;
        self.reader.trim_text(true);
        loop {
            match self.reader.read_event(&mut self.buf) {
                Ok(Event::Start(ref e)) => {
                    if let b"character" = e.name() {
                        counter += 1;
                    }
                }

                // Done after EOF
                Ok(Event::Eof) => break,

                // Break an return on errors
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }

        Ok(counter)
    }

    /// Parse a kanjidict2 xml file
    fn parse<F>(mut self, mut f: F) -> Result<Self, Error>
    where
        F: FnMut(Character, usize) -> bool,
    {
        self.reader.trim_text(true);
        let mut counter: usize = 0;

        loop {
            match self.reader.read_event(&mut self.buf) {
                // Parse each entry
                Ok(Event::Start(ref e)) => {
                    if let b"character" = e.name() {
                        // run callback with parsed entity
                        if f(self.parse_entry()?, counter) {
                            break;
                        }

                        counter += 1;
                    }
                }

                // Done after EOF
                Ok(Event::Eof) => break,

                // Break an return on errors
                Err(e) => return Err(e.into()),

                _ => (),
            }
        }

        Ok(self)
    }
}

impl<R> Parser<R>
where
    R: BufRead,
{
    /// Parses a whole single entry
    fn parse_entry(&mut self) -> Result<Character, Error> {
        /*
         * Define some inner entry, global variables in order to allow
         * the stream to get parsed. In each XML:Start event, all changing
         * variables are resetted. This prevents unecessary reallocation and
         * makes parsing easier.
         */
        let mut character = Character::default();

        /*
         * The stack represents the current 'history' of tags which have
         * been streamed before.
         */
        let mut stack: Vec<Tag> = Vec::new();

        loop {
            match self.reader.read_event(&mut self.buf)? {
                // Some tag was opened
                Event::Start(start) => {
                    let tag =
                        Tag::from_str(str::from_utf8(start.name())?, Some(start.attributes()))?;

                    stack.push(tag);
                }

                // Some tag was closed
                Event::End(end) => {
                    let tag = Tag::from_str(str::from_utf8(end.name())?, None)?;

                    if !stack.is_empty() && stack.last().unwrap().equals(&tag) {
                        stack.pop();
                    }

                    // Exit the loop if the entry is done getting parsed
                    if end.name() == b"character" {
                        break;
                    }
                }

                // Received some text
                Event::Text(text) => {
                    if let Some(tag) = stack.last() {
                        let value = text.unescape_and_decode(&self.reader)?;
                        character_apply_tag(&mut character, tag, value)?;
                    }
                }

                _ => (),
            }
        }

        Ok(character)
    }
}

/// Apply a given Tag to the Entry
fn character_apply_tag(element: &mut Character, tag: &Tag, value: String) -> Result<(), Error> {
    match *tag {
        Tag::Literal => {
            // Its always only one char
            element.literal = value.chars().into_iter().next().unwrap()
        }
        Tag::Jlpt => element.jlpt = Some(value.parse()?),
        Tag::Grade => element.grade = Some(value.parse()?),
        Tag::StrokeCount => element.stroke_count = value.parse()?,
        Tag::Variant => element.variant.push(value),
        Tag::Frequency => element.frequency = Some(value.parse()?),
        Tag::Nanori => element.natori.push(value),
        Tag::Meaning(is_jp) => {
            if is_jp {
                element.meaning.push(value)
            }
        }
        Tag::RadValue(v) => {
            if v && element.radical.is_none() {
                element.radical = value.parse().ok();
            }
        }

        Tag::Reading(ref r) => match r {
            ReadingType::JapaneseOn => element.on_readings.push(value),
            ReadingType::JapaneseKun => element.kun_readings.push(value),
            ReadingType::KoreanRomanized => element.korean_romanized.push(value),
            ReadingType::KoreanHangul => element.korean_hangul.push(value),
            ReadingType::Chinese => element.chinese_readings.push(value),
            _ => (),
        },
        _ => (),
    }
    Ok(())
}

/// An XML tag
#[derive(PartialEq)]
enum Tag {
    Character,
    Literal,
    Codepoint,
    Radical,
    RadValue(bool),
    RadName,
    Misc,
    Grade,
    StrokeCount,
    Frequency,
    Jlpt,
    DictNumber,
    DictRef,
    QueryCode,
    Reading(ReadingType),
    ReadingMeaning,
    Rmgroup,
    Variant,
    Meaning(bool),
    Nanori,
    CpValue,
    QCode,
}

#[derive(PartialEq)]
enum ReadingType {
    JapaneseOn,
    JapaneseKun,
    KoreanRomanized,
    KoreanHangul,
    Chinese,
    Other,
    None,
}

impl ReadingType {
    fn from_attributes(attributes: Attributes) -> Result<Self, Error> {
        let r_type = attributes
            .into_iter()
            .filter(|i| i.is_ok())
            .map(|i| i.unwrap())
            .filter(|i| str::from_utf8(i.key).unwrap() == "r_type")
            .map(|i| String::from_utf8(i.value.to_vec()).unwrap())
            .next()
            .ok_or(Error::ParseError)?;

        Ok(match r_type.as_str() {
            "ja_on" => Self::JapaneseOn,
            "ja_kun" => Self::JapaneseKun,
            "korean_r" => Self::KoreanRomanized,
            "korean_h" => Self::KoreanHangul,
            "pinyin" => Self::Chinese,
            _ => Self::Other,
        })
    }
}

impl Tag {
    // Custom equals method to ignore Tag::Reading
    // values for comparison
    fn equals(&self, other: &Self) -> bool {
        match self {
            Tag::Reading(_) => other.is_reading(),
            _ => self == other,
        }
    }

    fn from_str(s: &str, attributes: Option<Attributes>) -> Result<Tag, Error> {
        Ok(match s {
            "character" => Tag::Character,
            "literal" => Tag::Literal,
            "codepoint" => Tag::Codepoint,
            "radical" => Tag::Radical,
            "rad_value" => {
                let val = if let Some(attr) = attributes {
                    attr.filter_map(|i| i.is_ok().then(|| i.unwrap())).any(|i| {
                        i.key == b"rad_type"
                            && str::from_utf8(&i.value.to_vec()).unwrap() == "classical"
                    })
                } else {
                    false
                };
                Tag::RadValue(val)
            }
            "rad_name" => Tag::RadName,
            "misc" => Tag::Misc,
            "grade" => Tag::Grade,
            "stroke_count" => Tag::StrokeCount,
            "freq" => Tag::Frequency,
            "jlpt" => Tag::Jlpt,
            "dic_number" => Tag::DictNumber,
            "dic_ref" => Tag::DictRef,
            "query_code" => Tag::QueryCode,
            "reading" => Tag::Reading({
                if let Some(attr) = attributes {
                    ReadingType::from_attributes(attr)?
                } else {
                    ReadingType::None
                }
            }),
            "reading_meaning" => Tag::ReadingMeaning,
            "rmgroup" => Tag::Rmgroup,
            "variant" => Tag::Variant,
            "meaning" => Tag::Meaning({
                if let Some(attr) = attributes {
                    // Return true if no m_lang tag was found
                    // this indicates the meaning is the japanese meaning
                    !attr
                        .into_iter()
                        .filter_map(|i| i.ok())
                        .map(|i| str::from_utf8(i.key).unwrap())
                        .any(|i| i == "m_lang")
                } else {
                    true
                }
            }),
            "nanori" => Tag::Nanori,
            "cp_value" => Tag::CpValue,
            "q_code" => Tag::QCode,
            _ => return Err(Error::ParseError),
        })
    }

    /// Returns `true` if the tag is [`Reading`].
    fn is_reading(&self) -> bool {
        matches!(self, Self::Reading(..))
    }
}
