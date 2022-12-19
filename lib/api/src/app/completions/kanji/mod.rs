pub mod meaning;
pub mod reading;

use search::query::{Query, QueryLang};
use types::api::app::completions::Response;
use wana_kana::to_romaji::to_romaji;

/// Returns kanji suggestions
pub(crate) fn suggestions(query: Query) -> Option<Response> {
    match query.q_lang {
        QueryLang::Foreign => meaning::suggestions(&query),
        QueryLang::Japanese => japanese_suggestions(&query),
        /*
        QueryLang::Korean => todo!(),
        QueryLang::Undetected => todo!(),
        */
        _ => None,
    }
}

fn japanese_suggestions(query: &Query) -> Option<Response> {
    let romaji = to_romaji(query.query_str.as_str());
    let mut suggestions = super::words::native::suggestions(&query, &romaji, &[])?;

    // romev entries without kanji
    suggestions.retain(|i| i.secondary.is_some());

    Some(Response {
        suggestions,
        ..Default::default()
    })
}
