pub mod search_radicals;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::error::Error;

pub struct Radical<'a> {
    pub id: i32,
    pub radical: char,
    pub alternative: Option<char>,
    pub stroke_count: i32,
    pub readings: Vec<&'a str>,
    pub translations: Vec<&'a str>,
    pub note: Option<&'a str>,
}

/// Parse a radicals file. Calls [`f`] for each radical in file with the parsed radical value
pub fn parse<F>(path: &str, mut f: F) -> Result<(), Error>
where
    F: FnMut(Radical),
{
    let file = File::open(path)?;
    let bufr = BufReader::new(file);

    for line in bufr.lines().map(|i| i.unwrap()) {
        if let Some(parsed) = parse_item(&line) {
            f(parsed);
        }
    }

    Ok(())
}

/// Parses a single line of radical info
pub fn parse_item(line: &str) -> Option<Radical> {
    let mut split = line.split('\t');

    let radical: char = split.next()?.chars().into_iter().next()?;
    let alternative: Option<char> = split.next()?.chars().into_iter().next();
    let id: i32 = split.next().and_then(|i| i.parse().ok())?;
    let stroke_count: i32 = split.next().and_then(|i| i.parse().ok())?;
    let readings = split.next()?.split('・').collect::<Vec<_>>();
    let translations = split
        .next()?
        .split(',')
        .map(|i| i.trim())
        .collect::<Vec<_>>();
    let note = split
        .next()
        .map(|i| i.trim())
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
