pub mod binary_search;
pub mod korean;

use itertools::Itertools;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{cmp::Ordering, ops::Sub};

/// Return true if both slices have the same elments without being stored to be in the same order
pub fn same_elements<T>(v1: &[T], v2: &[T]) -> bool
where
    T: PartialEq,
{
    if v1.len() != v2.len() {
        return false;
    }

    for i in v1 {
        if !v2.contains(i) {
            return false;
        }
    }

    true
}

/// Return true if [`v1`] is a subset of [`v2`]
pub fn part_of<T>(v1: &[T], v2: &[T]) -> bool
where
    T: PartialEq,
{
    if v1.len() > v2.len() || v1.is_empty() {
        return false;
    }

    for i in v1 {
        if !v2.contains(i) {
            return false;
        }
    }

    true
}

/// Returns the cutset of both slices as newly allocated vector
pub fn union_elements<'a, T>(v1: &'a [T], v2: &'a [T]) -> Vec<&'a T>
where
    T: PartialEq,
{
    v1.iter().filter(|i| v2.contains(i)).collect::<Vec<_>>()
}

/// Get the relative order of two elements within a vector requires that a, b being element of vec
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
#[inline]
pub fn real_string_len(s: &str) -> usize {
    // We should probably use grapheme clusters here
    s.chars().count()
}

/// Returns an antisymmetric ordering of [`a`] and [`b`] where `a == true` < `b == true`
/// Example:
///
/// let a = true;
/// let b = false;
/// assert_eq!(bool_ord(a, b), Ordering::Less);
#[inline]
pub fn bool_ord(a: bool, b: bool) -> Ordering {
    if a && !b {
        Ordering::Less
    } else if !a && b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Returns `None` if the vec is empty or Some(Vec<T>) if not
#[inline]
pub fn to_option<T>(vec: Vec<T>) -> Option<Vec<T>> {
    (!vec.is_empty()).then(|| vec)
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

/// Remove duplicates from a vector and return a newly allocated one using a func to compare both
/// items. This doesn't need the source
/// vector to be sorted unlike `.dedup()`. Therefore it's heavier in workload
pub fn remove_dups_by<T, F>(inp: Vec<T>, eq: F) -> Vec<T>
where
    T: PartialEq,
    F: Fn(&T, &T) -> bool,
{
    let mut new: Vec<T> = Vec::new();

    for item in inp {
        if !contains(&new, &item, &eq) {
            new.push(item)
        }
    }

    new
}

pub fn contains<T, F>(inp: &[T], item: &T, eq: F) -> bool
where
    F: Fn(&T, &T) -> bool,
{
    for i in inp {
        if eq(i, item) {
            return true;
        }
    }
    false
}

/// Remove duplicates from a vector and return a newly allocated one. This doesn't need the source
/// vector to be sorted unlike `.dedup()`. Therefore it's heavier in workload
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

/// Returns an iterator over bools for each [`substr`] within [`text`] with the value `true` if the
/// given substr occurence is within [`open`] and [`close`] or not
///
/// Example:
///
/// is_surrounded_by(r#"this "is" an example"#, "is", '"','"')
///
/// => will return an iterator over [ false, true ]
///
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

/// Returns true if [`s`] represents [`c`]
pub fn char_eq_str(c: char, s: &str) -> bool {
    let mut chars = s.chars();
    let is = chars.next().map(|i| i == c).unwrap_or_default();
    is && chars.next().is_none()
}

/// Makes the first character to uppercase and returns a newly owned string
pub fn first_letter_upper(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

/// Returns a random alpha numeric string with the length of [`len`]
#[inline]
pub fn rand_alpha_numeric(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

/// Calculates the difference between `a` and `b`. This method never fails
#[inline]
pub fn diff<T: Sub<Output = T> + Ord>(a: T, b: T) -> T {
    if a > b {
        a - b
    } else {
        b - a
    }
}

/// Formats romaji text by removing all 'n' occurences of n+ for 1 < |n| <= 4
#[inline]
pub fn format_romaji_nn(inp: &str) -> String {
    inp.replace("nn", "ん")
        .replace("n'", "ん")
        .replace("nnn", "nn")
        .replace("nnnn", "nnn")
        .replace("nnnnn", "nnnn")
}
