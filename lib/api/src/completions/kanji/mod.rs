pub mod meaning;
pub mod reading;

use search::query::{Query, QueryLang};
use types::api::completions::Response;

/// Returns kanji suggestions
pub(crate) fn suggestions(query: Query) -> Option<Response> {
    match query.language {
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
    let romaji = romaji::RomajiExt::to_romaji(query.query.as_str());
    let mut suggestions = super::words::native::suggestions(&query, &romaji, &[])?;

    // romev entries without kanji
    suggestions.retain(|i| i.secondary.is_some());

    Some(Response {
        suggestions,
        ..Default::default()
    })
}
