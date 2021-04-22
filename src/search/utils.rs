use typed_igo::wordclass::*;

use crate::japanese::JapaneseExt;

/// Removes particles from inp text
pub fn remove_by_wordclass<P>(inp: &str, mut p: P) -> String
where
    P: FnMut(&WordClass) -> bool,
{
    let parsed = crate::JA_NL_PARSER.parse(inp);
    println!("{:#?}", parsed);

    parsed
        .iter()
        .filter_map(|i| (!p(&i.wordclass)).then(|| i.surface))
        .collect::<Vec<_>>()
        .join("")
}

pub fn trim_jp(inp: &str) -> String {
    inp.chars()
        .into_iter()
        .filter(|i| !i.is_japanese())
        .collect()
}

pub fn trim_non_jp(inp: &str) -> String {
    inp.chars()
        .into_iter()
        .filter(|i| i.is_japanese())
        .collect()
}

pub fn parse_jp_query(inp: &str) -> String {
    let inp = trim_non_jp(inp);

    // Workaround for now. Just remove all particles from jp input
    let query = remove_by_wordclass(&inp, |m| matches!(m, WordClass::Symbol(..),));

    // TODO do a proper query parsing here

    query
}

pub fn parse_foreign_query(inp: &str) -> String {
    let inp = trim_jp(inp);

    inp
}
