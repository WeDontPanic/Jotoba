use error::Error;

use crate::query::Query;

use super::result::Item;

pub fn search(query: &Query) -> Result<Vec<Item>, Error> {
    let genki_lesson = query
        .tags
        .iter()
        .find(|i| i.is_genki_lesson())
        .and_then(|i| i.as_genki_lesson());

    if genki_lesson.is_none() {
        return Ok(Vec::new());
    }

    let kanji_retrieve = resources::get().kanji();

    let genki_lesson = kanji_retrieve.by_genki_lesson(*genki_lesson.unwrap());

    if genki_lesson.is_none() {
        return Ok(Vec::new());
    }

    let kanji = genki_lesson
        // we ensured that there is a genki lesson above
        .unwrap()
        .iter()
        .filter_map(|literal| kanji_retrieve.by_literal(*literal))
        .cloned()
        .collect::<Vec<_>>();

    Ok(super::to_item(kanji, query))
}
