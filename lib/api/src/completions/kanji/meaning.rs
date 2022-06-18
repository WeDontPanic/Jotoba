use super::super::{words::foreign::try_romaji, Response};
use crate::completions::convert_results;
use autocompletion::suggest::{
    extension::{ngram::NGramExtension, similar_terms::SimilarTermsExtension},
    query::SuggestionQuery,
    task::SuggestionTask,
};
use search::query::Query;

/// Returns kanji meaning suggestions
pub fn suggestions(query: &Query) -> Option<Response> {
    let index = indexes::get_suggestions().kanji_meanings();

    let mut suggestion_task = SuggestionTask::new(30);

    let mut def_query = SuggestionQuery::new(index, &query.query);
    let mut ng_ext = NGramExtension::new(index);
    ng_ext.options.weights.freq_weight = 0.5;
    ng_ext.options.weights.total_weight = 0.7;
    def_query.add_extension(ng_ext);

    suggestion_task.add_query(def_query);

    if let Some(hira_query) = try_romaji(&query.query) {
        let jp_index = indexes::get_suggestions().jp_words();
        let mut rom_sug_query = SuggestionQuery::new(jp_index, hira_query);
        rom_sug_query.weights.total_weight = 0.5;

        let mut similar_terms = SimilarTermsExtension::new(jp_index, 4);
        similar_terms.options.weights.total_weight = 0.2;
        rom_sug_query.add_extension(similar_terms);

        suggestion_task.add_query(rom_sug_query);
    }

    let suggestions = convert_results(suggestion_task.search());
    Some(Response::new(suggestions))
}
