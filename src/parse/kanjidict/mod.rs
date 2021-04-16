use std::io::BufRead;
use std::str;
use std::str::FromStr;

use quick_xml::events::Event;
use quick_xml::Reader;
use strum_macros::{AsRefStr, Display, EnumString};

use super::parser::Parse;
use crate::error::Error;

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
    fn parse<F>(mut self, f: F) -> Result<Self, Error>
    where
        F: Fn(Character, usize) -> bool,
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
        let mut entry = Character::default();

        /*
         * The stack represents the current 'history' of tags which have
         * been streamed before.
         */
        let mut stack: Vec<Tag> = Vec::new();

        loop {
            match self.reader.read_event(&mut self.buf)? {
                // Some tag was opened
                Event::Start(start) => {
                    let tag = Tag::from_str(str::from_utf8(start.name())?)?;

                    stack.push(tag);
                }

                // Some tag was closed
                Event::End(end) => {
                    let tag = Tag::from_str(str::from_utf8(end.name())?)?;

                    if !stack.is_empty() && *stack.last().unwrap() == tag {
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

                        match tag {
                            _ => (),
                        }
                    }
                }

                // Empty tags <tag/>
                Event::Empty(val) => {
                    let tag = Tag::from_str(str::from_utf8(val.name())?);
                }

                _ => (),
            }
        }

        Ok(entry)
    }
}

/// An dict entry. Represents one word, phrase or expression
#[derive(Debug, Default, Clone)]
pub struct Character {}

impl Character {
    /// Apply a given Tag to the Entry
    fn apply_tag(&mut self, tag: &Tag, value: String) -> Result<(), Error> {
        #[allow(clippy::clippy::single_match)]
        match *tag {
            _ => (),
        }
        Ok(())
    }
}

/// An XML tag
#[derive(Debug, Clone, PartialEq, AsRefStr, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
enum Tag {
    Character,
    Literal,
    Codepoint,
    Radical,
    RadValue,
    RadName,
    Misc,
    Grade,
    StrokeCount,
    Frequency,
    JLPT,
    DictNumber,
    DictRef,
    QueryCode,
    ReadingMeaning,
    RmGroup,
    Variant,
    Meaning,
    Nanori,
}
