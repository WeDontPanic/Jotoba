use std::cmp::Ordering;

/*
/// Removes particles from inp text
pub fn remove_by_wordclass<P>(inp: &str, mut p: P) -> String
where
    P: FnMut(&WordClass) -> bool,
{
    let parsed = crate::JA_NL_PARSER.parse(inp);
    println!("{:#?}", parsed);

    parsed
        .iter()
        .filter_map(|i| (!p(&i.wordclass)).then(|| i.surface))
        .collect::<Vec<_>>()
        .join("")
}

pub fn trim_jp(inp: &str) -> String {
    inp.chars()
        .into_iter()
        .filter(|i| !i.is_japanese())
        .collect()
}

pub fn trim_non_jp(inp: &str) -> String {
    inp.chars()
        .into_iter()
        .filter(|i| i.is_japanese())
        .collect()
}
*/

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

/// Compares two levenshtein values
pub fn levenshtein_cmp(a: usize, b: usize) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Remove duplicates from a vector and return a
/// newly allocated one
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
