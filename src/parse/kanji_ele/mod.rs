use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MAPPINGS: &[(char, char)] = &[
    ('化', '⺅'),
    ('个', '儿'),
    ('乞', '乙'),
    ('并', '干'),
    ('邑', '⻏'),
    ('刈', '⺉'),
    ('阡', '⻖'),
    ('込', '⻌'),
    ('尚', '⺌'),
    ('忙', '⺖'),
    ('扎', '手'),
    ('汁', '⺡'),
    ('犯', '⺨'),
    ('艾', '⺾'),
    ('邦', '⻏'),
    ('老', '⺹'),
    ('杰', '⺣'),
    ('礼', '⺭'),
    ('疔', '疒'),
    ('禹', '禸'),
    ('初', '⻂'),
    ('買', '⺲'),
    ('滴', '啇'),
];

pub struct KanjiPart {
    pub radical: char,
    pub parts: Vec<char>,
}

/// Parses a kanji element file
pub fn parse(path: &str) -> impl Iterator<Item = KanjiPart> {
    let file = File::open(path).unwrap();

    BufReader::new(file)
        .lines()
        .map(|i| i.unwrap())
        .filter(|i| !i.starts_with('#'))
        .filter_map(|i| parse_item(&i))
}

/// Parses a single Kanji with its elements (one line)
fn parse_item(line: &str) -> Option<KanjiPart> {
    let mut split = line.split(':');

    let radical: char = split.next()?.chars().next()?;

    let parts = split
        .next()?
        .chars()
        .into_iter()
        // Filter out spaces
        .filter(|i| *i != ' ')
        .map(map_character)
        .collect();

    Some(KanjiPart { radical, parts })
}

/// Maps characters to the ones we need using [`MAPPINGS`]
fn map_character(inp: char) -> char {
    MAPPINGS
        .iter()
        .find(|i| i.0 == inp)
        .map(|i| i.1)
        .unwrap_or(inp)
}
