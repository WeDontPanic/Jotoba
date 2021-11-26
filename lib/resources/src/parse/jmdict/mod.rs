pub mod dialect;
pub mod field;
pub mod foreign_language;
pub mod gtype;
pub mod information;
pub mod languages;
pub mod misc;
pub mod part_of_speech;
pub mod priority;

use std::str::FromStr;
use std::{collections::HashMap, convert::TryFrom, str};
use std::{fmt::Display, io::BufRead};

use quick_xml::events::{attributes::Attributes, Event};
use quick_xml::Reader;
use regex::Regex;

use dialect::Dialect;
use field::Field;
use gtype::GType;
use information::Information;
use languages::Language;
use misc::Misc;
use part_of_speech::PartOfSpeech;
use priority::Priority;
use serde::{Deserialize, Serialize};

use crate::parse::{error::Error, parser::Parse};

use self::foreign_language::ForeignLanguage;

/// An dict entry. Represents one word, phrase or expression
#[derive(Debug, Default, Clone)]
pub struct Entry {
    pub sequence: u32,
    /// Different readings of a word
    pub elements: Vec<EntryElement>,
    /// Translations into various languages
    pub senses: Vec<EntrySense>,
}

/// A single element for an entry. Defines reading, kanji and additional
/// information for the japanese word
#[derive(Debug, Default, Clone)]
pub struct EntryElement {
    /// Is kanji reading
    pub kanji: bool,
    /// The reading
    pub value: String,
    pub priorities: Vec<Priority>,
    pub reading_info: Vec<Information>,
    pub no_true_reading: bool,
}

/// A single 'sense' item for an entry
#[derive(Debug, Default, Clone)]
pub struct EntrySense {
    pub glosses: Vec<GlossValue>,
    pub misc: Option<Misc>,
    pub part_of_speech: Vec<PartOfSpeech>,
    pub antonym: Option<String>,
    pub field: Option<Field>,
    pub xref: Option<String>,
    pub dialect: Option<Dialect>,
    pub information: Option<String>,
    pub gairaigo: Option<Gairaigo>,
    pub example_sentence: Option<u32>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Hash)]
pub struct Translation {
    pub language: Language,
    pub value: String,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Hash)]
pub struct Gairaigo {
    pub language: ForeignLanguage,
    pub fully_derived: bool,
    pub original: String,
}

/// A single gloss entry.
#[derive(Debug, Clone, PartialEq)]
pub struct GlossValue {
    pub language: Language,
    pub g_type: Option<GType>,
    pub value: String,
}

/// A jmdict parser
pub struct Parser<R>
where
    R: BufRead,
{
    reader: Reader<R>,
    buf: Vec<u8>,
    pub entity_mappings: HashMap<String, String>, // Available after parsing
}

impl<R> Parse<R, Entry> for Parser<R>
where
    R: BufRead,
{
    /// Create a new parser
    fn new(r: R) -> Parser<R> {
        Self {
            reader: Reader::from_reader(r),
            entity_mappings: HashMap::new(),
            buf: Vec::new(),
        }
    }

    /// Parse a jmdict xml file
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

    /// Parse a jmdict xml file
    fn parse<F>(mut self, mut f: F) -> Result<Self, Error>
    where
        F: FnMut(Entry, usize) -> bool,
    {
        self.reader.trim_text(true);
        let mut custom_entities: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let entity_re = Regex::new(r#"<!ENTITY\s+([^ \t\r\n]+)\s+"([^"]*)"\s*>"#).unwrap();
        let mut counter: usize = 0;

        loop {
            match self.reader.read_event(&mut self.buf) {
                // Parse custom entities
                Ok(Event::DocType(ref e)) => {
                    for cap in entity_re.captures_iter(&e.unescape_and_decode(&self.reader)?) {
                        custom_entities
                            .insert(cap[1].as_bytes().to_vec(), cap[1].as_bytes().to_vec());

                        self.entity_mappings.insert(
                            String::from_utf8(cap[1].as_bytes().to_vec())?,
                            String::from_utf8(cap[2].as_bytes().to_vec())?,
                        );
                    }
                }

                // Parse each entry
                Ok(Event::Start(ref e)) => {
                    if let b"entry" = e.name() {
                        // run callback with parsed entity
                        if f(self.parse_entry(&custom_entities)?, counter) {
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
    fn parse_entry(&mut self, custom_entities: &HashMap<Vec<u8>, Vec<u8>>) -> Result<Entry, Error> {
        /*
         * Define some inner entry, global variables in order to allow
         * the stream to get parsed. In each XML:Start event, all changing
         * variables are resetted. This prevents unecessary reallocation and
         * makes parsing easier.
         */
        let mut entry = Entry::default();
        let mut element = EntryElement::default();
        let mut sense = EntrySense::default();

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
                        Tag::from_str(str::from_utf8(start.name())?, Some(start.attributes()));

                    // Clear necessary items for new usage
                    if tag == Tag::KEle || tag == Tag::REle {
                        element.clear();
                    }
                    if tag == Tag::Sense {
                        sense.clear();
                    }

                    stack.push(tag);
                }

                // Some tag was closed
                Event::End(end) => {
                    let tag = Tag::from_str(str::from_utf8(end.name())?, None);

                    // If one of these tags are closed, apped its value to
                    // the current entry
                    match tag {
                        // Push parsed element
                        Tag::KEle | Tag::REle => entry.elements.push(element.clone()),
                        // Push parsed sense
                        Tag::Sense => {
                            if !sense.glosses.is_empty() {
                                entry.senses.push(sense.clone())
                            }
                        }
                        _ => (),
                    }

                    if !stack.is_empty() && *stack.last().unwrap() == tag {
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
                            custom_entities,
                        )?;

                        match tag {
                            // Elements
                            Tag::Keb | Tag::Reb => {
                                element.value = value;
                                element.kanji = *tag == Tag::Keb;
                            }
                            Tag::KePri | Tag::RePri => {
                                if let Ok(priority) = Priority::try_from(value.as_str()) {
                                    element.priorities.push(priority)
                                }
                            }
                            Tag::KeInf | Tag::ReInf => {
                                element.reading_info.push(Information::from_str(&value)?)
                            }

                            // Senses
                            Tag::Gloss(gloss) => {
                                let mut closs = gloss.clone();
                                closs.value = value;
                                sense.glosses.push(closs);
                            }
                            Tag::Pos => {
                                // We don't need all part_of_speech variants so we handle only
                                // supported ones here
                                if let Ok(pos) = PartOfSpeech::try_from(value.as_str()) {
                                    if !sense.part_of_speech.contains(&pos) {
                                        sense.part_of_speech.push(pos)
                                    }
                                }
                            }
                            Tag::Misc => sense.misc = Some(Misc::from_str(&value)?),
                            Tag::Ant => sense.antonym = Some(value),
                            Tag::Field => sense.field = Some(Field::from_str(&value)?),
                            Tag::Xref => sense.xref = Some(value),
                            Tag::Dialect => sense.dialect = Some(Dialect::from_str(&value)?),
                            Tag::SInf => sense.information = Some(value),
                            Tag::ExampleSrcID(src) => {
                                if src == "tat" {
                                    let id: u32 = value.parse()?;
                                    sense.example_sentence = Some(id);
                                }
                            }
                            Tag::LSource(gairaigo) => {
                                let mut gairaigo = gairaigo.clone();
                                gairaigo.original = value;
                                sense.gairaigo = Some(gairaigo)
                            }

                            // Other
                            _ => entry.apply_tag(tag, value)?,
                        }
                    }
                }

                // Empty tags <tag/>
                Event::Empty(val) => {
                    let tag = Tag::from_str(str::from_utf8(val.name())?, Some(val.attributes()));

                    if let Tag::ReNoKanji = tag {
                        element.no_true_reading = true
                    }
                }

                _ => (),
            }
        }

        Ok(entry)
    }
}

impl Entry {
    /// Apply a given Tag to the Entry
    fn apply_tag(&mut self, tag: &Tag, value: String) -> Result<(), Error> {
        #[allow(clippy::collapsible_match)]
        match *tag {
            Tag::EntSeq => self.sequence = value.parse()?,
            _ => (),
        }
        Ok(())
    }
}

impl EntryElement {
    #[inline]
    fn clear(&mut self) {
        self.kanji = false;
        self.value.clear();
        self.priorities.clear();
        self.reading_info.clear();
    }
}

impl EntrySense {
    fn clear(&mut self) {
        self.glosses.clear();

        if let Some(ref mut ant) = self.antonym {
            ant.clear();
            self.antonym = None;
        }

        if let Some(ref mut information) = self.information {
            information.clear();
            self.information = None;
        }

        if let Some(ref mut xref) = self.xref {
            xref.clear();
            self.xref = None;
        }

        self.field = None;
        self.dialect = None;
        self.misc = None;
        self.part_of_speech.clear();
        self.example_sentence = None;
        self.gairaigo = None;
    }
}

/// An XML tag
#[derive(Debug, Clone, PartialEq)]
enum Tag {
    EntSeq,                    // ent_seq Unique sequence of an entry
    KEle,                      // k_ele Kanji element. This is the Entry
    REle,      // r_ele reading element. This is the Entry if a word is written entirely in kana
    Keb,       // keb Contains a word or short phrase with at least one kanji
    Reb,       // reb
    KePri,     // ke_pri relative priority
    RePri,     // re_pri same as ke_pri
    KeInf,     // ke_inf coded information related to the keb
    ReNoKanji, // re_nokanji Represents that the reb/keb cannot be regarded as a true reading of the kanji
    ReRestr,   // re_restr Indicates reading only applies to to a subset of the keb elements
    ReInf,     // re_inf indicates unusal readings
    Sense,     // sense array of translational equivalents to the japanese word
    Stagk,     //
    Stagr,     //
    Xref,      // xref Indicates cross reference
    Ant,       // ant indicates another entry which is an antonym of the current entry/sense
    Pos,       // pos Part-of-Speech
    Field,     // field Information about the field of application of the entry/sense
    Misc,      // misc Other relevant information about entry/sense
    LSource(Gairaigo), // lsource indicates information about the source language
    Dialect, // dial For words specifically associated with regional dialects in Japanese, the entity code for that dialect, e.g. ksb for Kansaiben
    Gloss(GlossValue), // gloss Represents trans language words
    Pri, // pri Highlights patricular target-language words which are strongly associated with the japanese word
    SInf, // s_inf sense information, for additional sense info
    Example, // Example sentence for a sense
    ExampleText, // Form of the term in the example sentence
    ExampleSrcID(String), // Example sentence ID from tatoeba
    ExampleSentence(Language), // The actual example sentence in any language

    Unknown, // Parsing error
}

impl GlossValue {
    pub fn new(attributes: Option<Attributes>) -> Self {
        let (g_type, language) = {
            attributes
                .map(|attributes| {
                    let map = attributes
                        .into_iter()
                        .filter(|i| i.is_ok())
                        .map(|i| i.unwrap())
                        .map(|i| {
                            (
                                String::from_utf8(i.key.to_vec()).unwrap(),
                                String::from_utf8(i.value.to_vec()).unwrap(),
                            )
                        })
                        .collect::<HashMap<String, String>>();

                    (
                        map.get("g_type")
                            .and_then(|gtype| GType::from_str(gtype.as_str()).ok()),
                        map.get("xml:lang")
                            .map(|i| Language::from_str(i.as_str()).unwrap_or(Language::English))
                            .unwrap_or(Language::English),
                    )
                })
                .unwrap_or((None, Language::English))
        };

        GlossValue {
            value: String::default(),
            language,
            g_type,
        }
    }
}

impl Tag {
    /// Parse an xml tag into a Tag
    fn from_str(s: &str, attributes: Option<Attributes>) -> Self {
        match s {
            "ent_seq" => Tag::EntSeq,
            "k_ele" => Tag::KEle,
            "r_ele" => Tag::REle,
            "keb" => Tag::Keb,
            "reb" => Tag::Reb,
            "ke_inf" => Tag::KeInf,
            "re_inf" => Tag::ReInf,
            "ke_pri" => Tag::KePri,
            "re_nokanji" => Tag::ReNoKanji,
            "re_restr" => Tag::ReRestr,
            "re_pri" => Tag::RePri,
            "sense" => Tag::Sense,
            "stagk" => Tag::Stagk,
            "stagr" => Tag::Stagr,
            "xref" => Tag::Xref,
            "ant" => Tag::Ant,
            "pos" => Tag::Pos,
            "field" => Tag::Field,
            "misc" => Tag::Misc,
            "lsource" => Tag::LSource(parse_gairaigo(attributes)),
            "dial" => Tag::Dialect,
            "gloss" => Tag::Gloss(GlossValue::new(attributes)),
            "pri" => Tag::Pri,
            "example" => Tag::Example,
            "ex_text" => Tag::ExampleText,
            "ex_srce" => Tag::ExampleSrcID(parse_ex_srce(attributes)),
            "ex_sent" => Tag::ExampleSentence(get_language(attributes)),
            "s_inf" => Tag::SInf,
            _ => Tag::Unknown,
        }
    }
}

fn parse_ex_srce(attributes: Option<Attributes>) -> String {
    attributes
        .and_then(|attributes| {
            attributes
                .into_iter()
                .filter_map(|i| i.is_ok().then(|| i.unwrap()))
                .find(|i| str::from_utf8(i.key).unwrap() == "exsrc_type")
                .and_then(|i| String::from_utf8(i.value.as_ref().to_vec()).ok())
        })
        .unwrap_or_default()
}

fn get_language(attributes: Option<Attributes>) -> Language {
    attributes
        .and_then(|attributes| {
            attributes
                .into_iter()
                .filter_map(|i| i.is_ok().then(|| i.unwrap()))
                .find(|i| str::from_utf8(i.key).unwrap() == "xml:lang")
                .and_then(|i| {
                    let val = str::from_utf8(i.value.as_ref()).unwrap();
                    Language::from_str(val).ok()
                })
        })
        .unwrap_or_default()
}

fn parse_gairaigo(attributes: Option<Attributes>) -> Gairaigo {
    let mut gairaigo = Gairaigo::default();

    if attributes.is_none() {
        return gairaigo;
    }

    for attribute in attributes
        .unwrap()
        .into_iter()
        .filter_map(|i| i.is_ok().then(|| i.unwrap()))
    {
        let key = str::from_utf8(attribute.key).unwrap();
        let val = str::from_utf8(&attribute.value).unwrap();

        match key {
            "xml:lang" => {
                gairaigo.language = ForeignLanguage::from_str(val).unwrap_or_default();
            }
            "ls_wasei" => gairaigo.fully_derived = val == "y",
            _ => continue,
        }
    }

    gairaigo
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
