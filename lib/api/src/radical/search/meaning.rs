use std::collections::HashSet;

use japanese::JapaneseExt;
use types::jotoba::languages::Language;

pub fn search(query: &str, language: Language) -> HashSet<char> {
    if query.len() < 2 {
        return HashSet::new();
    }

    let mut res = search::radical::meaning_search(query);

    if res.len() > 4 {
        return res;
    }

    if japanese::guessing::could_be_romaji(query) {
        res.extend(super::jp_search::search(&query.to_hiragana()));
    } else {
        //res.extend(word_search(query, language));
        let fw_search = search::radical::word::ForeignSearch::new(query, language);
        res.extend(fw_search.run())
    }

    res
}
