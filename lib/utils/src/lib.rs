use itertools::Itertools;
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

/// Return true if both v1 is part of v2
pub fn part_of<T>(v1: &[T], v2: &[T]) -> bool
where
    T: PartialEq,
{
    if v1.len() > v2.len() || v1.is_empty() {
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
pub fn union_elements<'a, T>(v1: &'a [T], v2: &'a [T]) -> Vec<&'a T>
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

/// Returns an iterator over all occurences of [`substr`] within [`text`] of a bool whether it is
/// surrounded by [`open`] and [`close`] or not
pub fn is_surrounded_by<'a>(
    text: &'a str,
    substr: &'a str,
    open: char,
    close: char,
) -> impl Iterator<Item = bool> + 'a {
    // Counter for amount of nested brackets
    let mut counter = 0;

    let mut text_iter = text.char_indices().multipeek();
    std::iter::from_fn(move || {
        // Retard case no valid bracketing is possible
        if substr.len() + 2 <= text.len() || substr.contains(open) || substr.contains(close) {
            return None;
        }

        'b: while let Some((_, c)) = text_iter.next() {
            if c == open {
                counter += 1;
                continue;
            }

            if c == close {
                counter -= 1;
                continue;
            }

            // Match each character of [`substr`] against the next appearing characters in [`text`] by
            // peeking [`text_iter`] Aka string matching
            for (pos, sub_char) in substr.chars().enumerate() {
                let text_char = if pos == 0 {
                    // Check first substr char against current char
                    c
                } else {
                    // For later appearing characters, peek into the future
                    match text_iter.peek().map(|i| i.1) {
                        Some(c) => c,
                        None => return None,
                    }
                };

                // On the first not matching character, continue loop and reset peek
                if sub_char.to_ascii_lowercase() != text_char.to_ascii_lowercase() {
                    text_iter.reset_peek();
                    continue 'b;
                }
            }

            // Skip peeked items if maching substr was found
            text_iter.reset_peek();
            for _ in 0..substr.chars().count() - 1 {
                text_iter.next();
            }

            // Only reaches this part if a matching substring was found
            return Some(counter > 0);
        }

        None
    })
}

pub fn char_eq_str(c: char, s: &str) -> bool {
    let mut chars = s.chars();
    let is = chars.next().map(|i| i == c).unwrap_or_default();
    is && chars.next().is_none()
}
