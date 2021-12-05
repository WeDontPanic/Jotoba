pub mod search_radicals;

use types::jotoba::kanji::DetailedRadical;
use utils::to_option;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

/// Parse a radicals file. Calls `f` for each radical in file with the parsed radical value
pub fn parse(path: &str) -> impl Iterator<Item = DetailedRadical> {
    let file = File::open(path).expect("Error opening radicals file");
    let bufr = BufReader::new(file);

    let mut lines = bufr.lines().map(|i| i.unwrap());

    iter::from_fn(move || parse_item(lines.next()?))
}

/// Parses a single line of radical info
pub fn parse_item(line: String) -> Option<DetailedRadical> {
    let mut split = line.split('\t');

    let literal: char = split.next()?.chars().into_iter().next()?;
    let alternative: Option<char> = split.next()?.chars().into_iter().next();
    let id: u16 = split.next().and_then(|i| i.parse().ok())?;
    let stroke_count: u8 = split.next().and_then(|i| i.parse().ok())?;
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

    Some(DetailedRadical {
        id,
        literal,
        alternative,
        stroke_count,
        readings,
        translations: to_option(translations),
    })
}
