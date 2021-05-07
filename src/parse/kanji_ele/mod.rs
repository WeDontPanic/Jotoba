use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Parses a kanji element file
pub fn parse(path: &str) -> impl Iterator<Item = KanjiPart> {
    let file = File::open(path).unwrap();

    BufReader::new(file)
        .lines()
        .map(|i| i.unwrap())
        .filter(|i| !i.starts_with("#"))
        .filter_map(|i| parse_item(&i))
}

#[derive(Debug, Clone, PartialEq)]
pub struct KanjiPart {
    pub radical: char,
    pub parts: Vec<char>,
}

/// Parses a single line of pitch accent info and returns a result in form of
/// Some((kanji, kana, [pitch])) or None if the line is invalid
fn parse_item(line: &str) -> Option<KanjiPart> {
    let mut split = line.split(":");

    let radical: char = split.next()?.chars().next()?;

    let parts = split
        .next()?
        .chars()
        .into_iter()
        .filter(|i| *i != ' ')
        .map(|i| if i == '｜' { '丨' } else { i })
        .collect();

    Some(KanjiPart { radical, parts })
}
