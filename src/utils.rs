use std::cmp::Ordering;

/// Return true if both vectors have the same elments
pub fn same_elements<T>(v1: &Vec<T>, v2: &Vec<T>) -> bool
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

/// Returns a vector of all items both vectors contain
pub fn union_elements<'a, T>(v1: &'a Vec<T>, v2: &'a Vec<T>) -> Vec<&'a T>
where
    T: PartialEq,
{
    v1.iter().filter(|i| v2.contains(i)).collect::<Vec<_>>()
}

/// Get the order of two elements in a vector
/// requires that a, b are element of vec
pub fn get_item_order<T>(vec: &Vec<T>, a: &T, b: &T) -> Option<Ordering>
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
    s.chars().count()
}

/// Retrns None if the vec is empty or Some(Vec<T>) if not
pub fn to_option<T>(vec: Vec<T>) -> Option<Vec<T>> {
    if vec.is_empty() {
        None
    } else {
        Some(vec)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_get_item_order_less() {
        let vec = vec!["1", "2", "3", "5"];
        let a = "1";
        let b = "2";
        assert_eq!(get_item_order(&vec, a, b), Some(Ordering::Less));
    }

    #[test]
    fn test_get_item_order_equal() {
        let vec = vec!["1", "2", "3", "5"];
        let a = "1";
        let b = "1";
        assert_eq!(get_item_order(&vec, a, b), Some(Ordering::Equal));
    }

    #[test]
    fn test_get_item_order_greater() {
        let vec = vec!["1", "2", "3", "5"];
        let a = "5";
        let b = "2";
        assert_eq!(get_item_order(&vec, a, b), Some(Ordering::Greater));
    }
}
