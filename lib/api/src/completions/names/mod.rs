use autocompletion::suggest::{query::SuggestionQuery, task::SuggestionTask};
use error::api_error::RestError;
use japanese::JapaneseExt;
use search::query::{Query, QueryLang};

use super::{
    convert_results,
    storage::{NAME_NATIVE, NAME_TRANSCRIPTIONS},
    Response,
};

/// Returns name suggestions
pub(crate) fn suggestions(query: Query) -> Result<Response, RestError> {
    Ok(match query.language {
        QueryLang::Japanese => native_suggestions(&query)?,
        QueryLang::Foreign => transcription_suggestions(&query)?,
        _ => Response::default(),
    })
}

/// Returns trascripted name suggestions
pub fn transcription_suggestions(query: &Query) -> Result<Response, RestError> {
    let query_str = &query.query;

    let index = match NAME_TRANSCRIPTIONS.get() {
        Some(v) => v,
        None => return Ok(Response::default()),
    };

    let mut task = SuggestionTask::new(30);

    task.add_query(SuggestionQuery::new(index, query_str));

    if let Some(romaji_query) = super::words::foreign::try_romaji(query_str) {
        if let Some(jp_index) = NAME_NATIVE.get() {
            task.add_query(SuggestionQuery::new(jp_index, romaji_query.clone()));

            let katakana = romaji::RomajiExt::to_katakana(romaji_query.as_str());
            if katakana != romaji_query {
                task.add_query(SuggestionQuery::new(index, katakana));
            }
        }
    }

    let suggestions = convert_results(task.search());
    Ok(Response {
        suggestions,
        ..Default::default()
    })
}

/// Returns native name suggestions
pub fn native_suggestions(query: &Query) -> Result<Response, RestError> {
    let query_str = &query.query;

    let index = match NAME_NATIVE.get() {
        Some(v) => v,
        None => return Ok(Response::default()),
    };

    let mut task = SuggestionTask::new(30);

    task.add_query(SuggestionQuery::new(index, query_str));

    let katakana = romaji::RomajiExt::to_katakana(query_str.as_str());
    if &katakana != query_str {
        task.add_query(SuggestionQuery::new(index, katakana));
    }

    let hiragana = query_str.to_hiragana();
    if &hiragana != query_str {
        task.add_query(SuggestionQuery::new(index, hiragana));
    }

    let suggestions = convert_results(task.search());
    Ok(Response {
        suggestions,
        ..Default::default()
    })
}
