use crate::error::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Parses a pitch info file
pub fn parse<F>(file: String, mut f: F) -> Result<(), Error>
where
    F: FnMut((String, String, Vec<i32>), i32, usize),
{
    let fa = File::open(&file)?;
    let bufr = BufReader::new(fa);
    let count = bufr.lines().count();

    let fb = File::open(&file)?;
    let bufr = BufReader::new(fb);

    let mut counter = 0;
    for line in bufr.lines().map(|i| i.unwrap()) {
        if let Some(d) = parse_item(line) {
            f(d, counter, count);
            counter += 1;
        }
    }

    Ok(())
}

/// Parses a single line of pitch accent info and returns a result in form of
/// Some((kaji, kana, [pitch])) or None if the line is invalid
pub fn parse_item(line: String) -> Option<(String, String, Vec<i32>)> {
    let mut split = line.split("\t");
    let kanji = split.next()?;
    let kana = split.next()?;
    let pitch = split.next()?;

    let pitch = pitch
        .split(",")
        .into_iter()
        .map(|i| i.parse::<i32>().ok())
        .collect::<Option<Vec<i32>>>()?;

    Some((kanji.to_owned(), kana.to_owned(), pitch))
}
