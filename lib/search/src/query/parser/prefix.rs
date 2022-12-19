use crate::query::prefix::SearchPrefix;
use std::str::FromStr;
use types::jotoba::language::Language;

/// Strinps and parses a `SearchPrefix` from a `query`
pub fn parse_prefix(query: &str) -> (&str, Option<SearchPrefix>) {
    if let (new_query, Some(lang)) = try_lang_prefix(query) {
        return (new_query, Some(SearchPrefix::LangOverwrite(lang)));
    }

    if let Some(seq_id) = try_sequence(query) {
        return (query, Some(SearchPrefix::BySequence(seq_id)));
    }

    (query, None)
}

fn try_lang_prefix(query: &str) -> (&str, Option<Language>) {
    let split_pos = query.find(':');
    if split_pos.is_none() || *split_pos.as_ref().unwrap() > 3 || query.len() < 5 {
        return (query, None);
    }

    let split_pos = split_pos.unwrap();

    let lang_str = &query[..split_pos].trim();

    let lang = match Language::from_str(lang_str) {
        Ok(lang) => lang,
        Err(_) => {
            return (query, None);
        }
    };

    let new_query = query[split_pos + 1..].trim();

    (new_query, Some(lang))
}

#[inline]
fn try_sequence(query: &str) -> Option<u32> {
    if let Some(seq_str) = query.strip_prefix("seq:") {
        let seq_str = seq_str.trim();
        let parsed: u32 = seq_str.parse().ok()?;
        return Some(parsed);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_override_split() {
        let query = "eng: dog";
        let (new_query, language) = try_lang_prefix(query);
        assert_eq!(new_query, "dog");
        assert_eq!(language, Some(Language::English));
    }

    #[test]
    fn test_lang_override_split_invalid() {
        let query = "eng:";
        let (new_query, language) = try_lang_prefix(query);
        assert_eq!(new_query, "eng:");
        assert_eq!(language, None);

        let query = "egn:";
        let (new_query, language) = try_lang_prefix(query);
        assert_eq!(new_query, "egn:");
        assert_eq!(language, None);
    }
}
