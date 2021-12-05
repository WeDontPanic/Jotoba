use std::{
    collections::HashMap,
    io::BufRead,
    str::{self, FromStr},
};

use quick_xml::{events::Event, Reader};
use regex::Regex;
use strum_macros::Display;
use types::{jotoba::names::name_type::NameType, raw::jmnedict::NameEntry};

use crate::parse::{error::Error, parser::Parse};

/// A jmnedict parser
pub struct Parser<R>
where
    R: BufRead,
{
    reader: Reader<R>,
    buf: Vec<u8>,
    entity_mappings: HashMap<Vec<u8>, Vec<u8>>, // Available after parsing
}

impl<R> Parse<R, NameEntry> for Parser<R>
where
    R: BufRead,
{
    /// Create a new parser
    fn new(r: R) -> Parser<R> {
        Self {
            reader: Reader::from_reader(r),
            buf: Vec::new(),
            entity_mappings: HashMap::new(),
        }
    }

    /// Parse a jmnedict xml file
    fn count(mut self) -> Result<usize, Error> {
        let mut counter = 0;
        self.reader.trim_text(true);
        loop {
            match self.reader.read_event(&mut self.buf) {
                Ok(Event::Start(ref e)) => {
                    if let b"entry" = e.name() {
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

    /// Parse a jmnedict xml file
    fn parse<F>(mut self, mut f: F) -> Result<Self, Error>
    where
        F: FnMut(NameEntry, usize) -> bool,
    {
        self.reader.trim_text(true);
        let entity_re = Regex::new(r#"<!ENTITY\s+([^ \t\r\n]+)\s+"([^"]*)"\s*>"#).unwrap();
        let mut counter: usize = 0;

        loop {
            match self.reader.read_event(&mut self.buf) {
                // Parse custom entities
                Ok(Event::DocType(ref e)) => {
                    for cap in entity_re.captures_iter(&e.unescape_and_decode(&self.reader)?) {
                        self.entity_mappings
                            .insert(cap[1].as_bytes().to_vec(), cap[1].as_bytes().to_vec());
                    }
                }

                // Parse each entry
                Ok(Event::Start(ref e)) => {
                    if let b"entry" = e.name() {
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
    fn parse_entry(&mut self) -> Result<NameEntry, Error> {
        /*
         * Define some inner entry, global variables in order to allow
         * the stream to get parsed. In each XML:Start event, all changing
         * variables are resetted. This prevents unecessary reallocation and
         * makes parsing easier.
         */
        let mut entry = NameEntry::default();

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

                    if !stack.is_empty() && stack.last().unwrap() == &tag {
                        stack.pop();
                    }
                    // Exit the loop if the entry is done getting parsed
                    if end.name() == b"entry" {
                        break;
                    }
                }

                // Received some text
                Event::Text(text) => {
                    if let Some(tag) = stack.last() {
                        let value = text.unescape_and_decode_with_custom_entities(
                            &self.reader,
                            &self.entity_mappings,
                        )?;

                        entry_apply_tag(&mut entry, tag, value)?;
                    }
                }

                _ => (),
            }
        }

        Ok(entry)
    }
}

/// Apply a given Tag to the Entry
fn entry_apply_tag(entry: &mut NameEntry, tag: &Tag, value: String) -> Result<(), Error> {
    match *tag {
        Tag::Sequence => entry.sequence = value.parse()?,
        Tag::Transcription => entry.transcription = value,
        Tag::Xref => entry.xref = Some(value),
        Tag::KanjiB => entry.kanji_element = Some(value),
        Tag::ReadingB => entry.kana_element = value,
        Tag::NameType => {
            let nt = NameType::from_str(&value)?;

            if let Some(ref mut arr) = entry.name_type {
                arr.push(nt);
            } else {
                entry.name_type = Some(vec![nt]);
            }
        }
        _ => (),
    }
    Ok(())
}

/// An XML tag
#[derive(Debug, Clone, Display, PartialEq)]
enum Tag {
    Entry,
    Sequence,

    KanjiElement,
    KanjiB,
    KanjiInfo,
    KanjiPriority,

    ReadingElement,
    ReadingB,
    ReadingReStr,
    ReadingInfo,
    ReadingPriority,
    Translation,
    NameType,
    Xref,
    Transcription,
}

impl Tag {
    fn from_str(s: &str) -> Result<Tag, Error> {
        Ok(match s {
            "entry" => Tag::Entry,
            "ent_seq" => Tag::Sequence,
            "k_ele" => Tag::KanjiElement,
            "keb" => Tag::KanjiB,
            "ke_inf" => Tag::KanjiInfo,
            "ke_pri" => Tag::KanjiPriority,
            "r_ele" => Tag::ReadingElement,
            "reb" => Tag::ReadingB,
            "re_restr" => Tag::ReadingReStr,
            "re_inf" => Tag::ReadingInfo,
            "re_pri" => Tag::ReadingPriority,
            "trans" => Tag::Translation,
            "name_type" => Tag::NameType,
            "xref" => Tag::Xref,
            "trans_det" => Tag::Transcription,
            _ => return Err(Error::ParseError),
        })
    }
}
