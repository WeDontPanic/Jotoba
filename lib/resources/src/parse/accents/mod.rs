use types::jotoba::accents::PitchItem;

use super::error::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

/// Parses a pitch info file and returns the amount of pitch items and an iteator over all items
pub fn parse(file: &str) -> Result<(usize, impl Iterator<Item = PitchItem>), Error> {
    let mut fa = File::open(file)?;
    let bufr = BufReader::new(&fa);
    let count = bufr.lines().count();

    fa.seek(SeekFrom::Start(0))?;
    let bufr = BufReader::new(fa);

    Ok((
        count,
        bufr.lines().map(|i| i.unwrap()).filter_map(parse_item),
    ))
}

/// Parses a single line of pitch accent info
pub fn parse_item(line: String) -> Option<PitchItem> {
    let mut split = line.split('\t');
    let kanji = split.next()?;
    let kana = split.next()?;
    let pitch = split.next()?;

    let pitch = pitch
        .split(',')
        .into_iter()
        .map(|i| i.parse::<i32>().ok())
        .collect::<Option<Vec<i32>>>()?;

    Some(PitchItem {
        pitch,
        kanji: kanji.to_owned(),
        kana: kana.to_owned(),
    })
}
