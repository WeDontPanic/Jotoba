use std::cmp::Ordering;

/// Get the order of two elements within a vector requires that a, b are element of vec
pub fn get_item_order<T>(vec: &[T], a: &T, b: &T) -> Option<Ordering>
where
    T: PartialEq,
{
    if a == b {
        return Some(Ordering::Equal);
    }

    for i in vec {
        if *i == *a {
            return Some(Ordering::Less);
        }
        if *i == *b {
            return Some(Ordering::Greater);
        }
    }

    None
}

/// Returns the real amount of characters in a string
pub fn real_string_len(s: &str) -> usize {
    // We should probably use grapheme clusters here
    s.chars().count()
}

/// Retrns None if the vec is empty or Some(Vec<T>) if not
pub fn to_option<T>(vec: Vec<T>) -> Option<Vec<T>> {
    (!vec.is_empty()).then(|| vec)
}

/// Returns an inverted Ordering of [`order`]
pub fn invert_ordering(order: Ordering) -> Ordering {
    match order {
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => Ordering::Less,
    }
}
