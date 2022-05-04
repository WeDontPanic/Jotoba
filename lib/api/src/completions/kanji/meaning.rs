use autocompletion::suggest::{
    extension::similar_terms::SimilarTermsExtension, query::SuggestionQuery, task::SuggestionTask,
};
use error::api_error::{Origin, RestError};
use search::query::Query;

use crate::completions::{convert_results, storage};

use super::super::{storage::K_MEANING_SUGGESTIONS, Response};

/// Returns kanji meaning suggestions
pub fn suggestions(query: &Query) -> Result<Response, RestError> {
    let index = K_MEANING_SUGGESTIONS
        .get()
        .ok_or(RestError::Missing(Origin::Suggestions))?;

    let mut suggestion_task = SuggestionTask::new(30);

    suggestion_task.add_query(SuggestionQuery::new(index, &query.query));

    if let Some(hira_query) = super::super::words::foreign::try_romaji(&query.query) {
        if let Some(jp_engine) = storage::JP_WORD_INDEX.get() {
            let mut rom_sug_query = SuggestionQuery::new(jp_engine, hira_query);
            rom_sug_query.weights.total_weight = 0.5;

            let mut similar_terms = SimilarTermsExtension::new(jp_engine, 4);
            similar_terms.options.weights.total_weight = 0.2;
            rom_sug_query.add_extension(similar_terms);

            suggestion_task.add_query(rom_sug_query);
        }
    }

    let suggestions = convert_results(suggestion_task.search());

    Ok(Response {
        suggestions,
        ..Default::default()
    })
}
