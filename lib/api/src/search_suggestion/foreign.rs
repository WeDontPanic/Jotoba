use super::*;

/// Returns suggestions based on non japanese input
pub(super) async fn suggestions(query: &Query, query_str: &str) -> Option<Vec<WordPair>> {
    let lang = query.settings.user_lang;

    // Get suggestion DB
    let suggestion_db = SUGGESTIONS.get()?;

    // Search for suggestions
    let results = suggestion_db.search(query_str, lang).await?;

    // Transforms results into WordPairs
    let res = results
        .into_iter()
        .map(|i| WordPair {
            primary: i.text.to_owned(),
            secondary: None,
        })
        .take(10)
        .collect();

    Some(res)
}
