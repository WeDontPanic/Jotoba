use error::Error;

use crate::query::Query;

use super::KanjiResult;

pub fn search(query: &Query) -> Result<KanjiResult, Error> {
    let genki_lesson = query
        .tags
        .iter()
        .find(|i| i.is_genki_lesson())
        .and_then(|i| i.as_genki_lesson());

    if genki_lesson.is_none() {
        return Ok(KanjiResult::default());
    }

    let kanji_retrieve = resources::get().kanji();

    let genki_lesson = kanji_retrieve.by_genki_lesson(*genki_lesson.unwrap());

    if genki_lesson.is_none() {
        return Ok(KanjiResult::default());
    }

    let kanji = genki_lesson
        // we ensured that there is a genki lesson above
        .unwrap()
        .iter()
        .filter_map(|literal| kanji_retrieve.by_literal(*literal))
        .cloned()
        .collect::<Vec<_>>();

    let len = kanji.len();

    let page_offset = query.page_offset(query.settings.kanji_page_size as usize);

    let kanji = kanji
        .into_iter()
        .skip(page_offset)
        .take(query.settings.kanji_page_size as usize)
        .collect::<Vec<_>>();

    let items = super::to_item(kanji, query);

    Ok(KanjiResult {
        items,
        total_items: len,
    })
}
