use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use types::jotoba::kanji::SearchRadical;

use crate::parse::error::Error;

/// Parses a search radicals file
pub fn parse(path: &str) -> Result<impl Iterator<Item = SearchRadical>, Error> {
    let file = File::open(path)?;

    Ok(BufReader::new(file)
        .lines()
        .map(|i| i.unwrap())
        .filter(|i| !i.starts_with('#'))
        .filter_map(|i| parse_item(&i))
        .flatten())
}

/// Parses a single line of pitch accent info and returns a result in form of
/// Some((kanji, kana, pitch)) or None if the line is invalid
pub fn parse_item(line: &str) -> Option<Vec<SearchRadical>> {
    let mut split = line.split(':');

    let stroke_count: i32 = split.next()?.parse().ok()?;

    Some(
        split
            .next()?
            .chars()
            .into_iter()
            .map(|rad| SearchRadical {
                stroke_count,
                radical: rad,
            })
            .collect(),
    )
}
