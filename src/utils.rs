use std::cmp::Ordering;

/// Return true if both vectors have the same elments
pub fn same_elements<T>(v1: &[T], v2: &[T]) -> bool
where
    T: PartialEq,
{
    if v1.len() != v2.len() {
        return false;
    }

    for i in v1 {
        if !v2.contains(&i) {
            return false;
        }
    }

    true
}

/// Returns a vector of all items which are part of both vectors
pub fn union_elements<'a, T>(v1: &'a Vec<T>, v2: &'a Vec<T>) -> Vec<&'a T>
where
    T: PartialEq,
{
    v1.iter().filter(|i| v2.contains(i)).collect::<Vec<_>>()
}

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

/// Returns an ordering based on the option variants.
/// Ordering: Some < None
/// In case both are equal, None gets returned
pub fn option_order<T>(a: &Option<T>, b: &Option<T>) -> Option<Ordering> {
    if a.is_some() && !b.is_some() {
        Some(Ordering::Less)
    } else if !a.is_some() && b.is_some() {
        Some(Ordering::Greater)
    } else {
        None
    }
}

/// Remove duplicates from a vector and return a newly allocated one
pub fn remove_dups<T>(inp: Vec<T>) -> Vec<T>
where
    T: PartialEq,
{
    let mut new: Vec<T> = Vec::new();

    for item in inp {
        if !new.contains(&item) {
            new.push(item)
        }
    }

    new
}
