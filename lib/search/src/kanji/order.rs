use utils::option_order;

use super::result::Item;

use std::cmp::Ordering;

/// Order kanji results which were found by the kanjis meaning appropriately
pub(crate) fn by_meaning(a: &Item, b: &Item) -> Ordering {
    let a = &a.kanji;
    let b = &b.kanji;

    if let Some(o) = option_order(&a.grade, &b.grade) {
        return o;
    }

    if let Some(o) = option_order(&a.frequency, &b.frequency) {
        return o;
    }

    if let Some(o) = option_order(&a.jlpt, &b.jlpt) {
        return o;
    }

    Ordering::Equal
}
