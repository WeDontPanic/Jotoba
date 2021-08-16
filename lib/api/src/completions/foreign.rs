use resources::models::suggestions::native_words::ForeignSuggestion;
use utils::remove_dups;

use super::*;

/// Returns suggestions based on non japanese input
pub(super) async fn suggestions(query: &Query, query_str: &str) -> Option<Vec<WordPair>> {
    let lang = query.settings.user_lang;

    // Get suggestion DB
    let suggestion_db = WORD_SUGGESTIONS.get()?;

    // Search for suggestions
    let results = suggestion_db.search(query_str, lang).await?;

    // Transforms results into WordPairs
    let res: Vec<_> = results
        .into_iter()
        .map(|i| WordPair {
            primary: i.text.to_owned(),
            secondary: None,
        })
        .take(10)
        .collect();

    let res = remove_dups(res);

    Some(res)
}

impl From<ForeignSuggestion> for WordPair {
    #[inline]
    fn from(suggestion: ForeignSuggestion) -> Self {
        WordPair {
            primary: suggestion.text.to_owned(),
            secondary: None,
        }
    }
}
