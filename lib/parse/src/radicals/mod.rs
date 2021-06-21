pub mod search_radicals;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Radical {
    pub id: i32,
    pub radical: char,
    pub alternative: Option<char>,
    pub stroke_count: i32,
    pub readings: Vec<String>,
    pub translations: Vec<String>,
    pub note: Option<String>,
}

/// Parse a radicals file. Calls [`f`] for each radical in file with the parsed radical value
pub fn parse(path: &str) -> impl Iterator<Item = Radical> {
    let file = File::open(path).expect("Error opening radicals file");
    let bufr = BufReader::new(file);

    let mut lines = bufr.lines().map(|i| i.unwrap());

    std::iter::from_fn(move || parse_item(lines.next()?))
}

/// Parses a single line of radical info
pub fn parse_item(line: String) -> Option<Radical> {
    let mut split = line.split('\t');

    let radical: char = split.next()?.chars().into_iter().next()?;
    let alternative: Option<char> = split.next()?.chars().into_iter().next();
    let id: i32 = split.next().and_then(|i| i.parse().ok())?;
    let stroke_count: i32 = split.next().and_then(|i| i.parse().ok())?;
    let readings = split
        .next()?
        .split('・')
        .map(|i| i.to_owned())
        .collect::<Vec<_>>();
    let translations = split
        .next()?
        .split(',')
        .map(|i| i.trim().to_owned())
        .collect::<Vec<_>>();
    let note = split
        .next()
        .map(|i| i.trim().to_owned())
        .and_then(|i| (!i.is_empty()).then(|| i));

    Some(Radical {
        id,
        radical,
        alternative,
        stroke_count,
        readings,
        translations,
        note,
    })
}
