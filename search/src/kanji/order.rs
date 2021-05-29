use utils::option_order;

use super::result::Item;

use std::cmp::Ordering;

/// Order kanji results which were found
/// by the kanjis meaning appropriately
pub(crate) fn by_meaning(a: &Item, b: &Item) -> Ordering {
    let a = &a.kanji;
    let b = &b.kanji;

    if let Some(o) = option_order(&a.kanji.grade, &b.kanji.grade) {
        return o;
    }

    if let Some(o) = option_order(&a.kanji.frequency, &b.kanji.frequency) {
        return o;
    }

    if let Some(o) = option_order(&a.kanji.jlpt, &b.kanji.jlpt) {
        return o;
    }

    Ordering::Equal
}
