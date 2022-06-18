use super::{convert_results, Response};
use autocompletion::suggest::{
    extension::ngram::NGramExtension, query::SuggestionQuery, task::SuggestionTask,
};
use japanese::JapaneseExt;
use search::query::{Query, QueryLang};

/// Returns name suggestions
pub(crate) fn suggestions(query: Query) -> Option<Response> {
    match query.language {
        QueryLang::Japanese => native_suggestions(&query),
        QueryLang::Foreign => transcription_suggestions(&query),
        _ => None,
    }
}

/// Returns trascripted name suggestions
pub fn transcription_suggestions(query: &Query) -> Option<Response> {
    let query_str = &query.query;
    let index = indexes::get_suggestions().names_foreign();

    let mut task = SuggestionTask::new(30);

    let mut def_query = SuggestionQuery::new(index, query_str);
    let ng_ext = NGramExtension::new(index);
    def_query.add_extension(ng_ext);

    task.add_query(def_query);

    if let Some(romaji_query) = super::words::foreign::try_romaji(query_str) {
        let jp_index = indexes::get_suggestions().names_native();
        task.add_query(SuggestionQuery::new(jp_index, romaji_query.clone()));

        let katakana = romaji::RomajiExt::to_katakana(romaji_query.as_str());
        if katakana != romaji_query {
            task.add_query(SuggestionQuery::new(index, katakana));
        }
    }

    let suggestions = convert_results(task.search());
    Some(Response::new(suggestions))
}

/// Returns native name suggestions
pub fn native_suggestions(query: &Query) -> Option<Response> {
    let query_str = &query.query;

    let index = indexes::get_suggestions().names_native();
    let mut task = SuggestionTask::new(30);

    let mut def_query = SuggestionQuery::new(index, query_str);
    let ng_ext = NGramExtension::new(index);
    def_query.add_extension(ng_ext);

    task.add_query(def_query);

    let katakana = romaji::RomajiExt::to_katakana(query_str.as_str());
    if &katakana != query_str {
        task.add_query(SuggestionQuery::new(index, katakana));
    }

    let hiragana = query_str.to_hiragana();
    if &hiragana != query_str {
        task.add_query(SuggestionQuery::new(index, hiragana));
    }

    let suggestions = convert_results(task.search());
    Some(Response {
        suggestions,
        ..Default::default()
    })
}
