use super::result::Item;
use std::cmp::Ordering;
use utils::option_order;

/// Order kanji results which were found by the kanjis meaning appropriately
#[inline]
pub(crate) fn default(a: &Item, b: &Item) -> Ordering {
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
