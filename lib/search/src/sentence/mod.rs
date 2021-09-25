pub mod result;

use super::query::Query;
use crate::{
    engine::{
        result::SearchResult,
        sentences::{foreign as foreign_engine, japanese as japanese_engine},
    },
    query::QueryLang,
};
use error::Error;
use resources::parse::jmdict::languages::Language;
use std::time::Instant;

/// Searches for sentences
pub async fn search(query: &Query) -> Result<(Vec<result::Item>, usize), Error> {
    let start = Instant::now();

    let lang = query.settings.user_lang;

    let res = match query.language {
        QueryLang::Japanese => japanese_documents(query).await,
        _ => foreign_documents(query).await,
    }?;

    let sentence_storage = resources::get().sentences();

    let sentences = res
        .retrieve_ordered(|i| sentence_storage.by_id(i))
        .filter_map(|i| {
            let sentence =
                result::Sentence::from_m_sentence(i.clone(), lang, query.settings.show_english)?;
            Some(result::Item { sentence })
        })
        .skip(query.page_offset)
        .take(query.settings.items_per_page as usize)
        .collect::<Vec<_>>();

    println!("Sentence search took: {:?}", start.elapsed());

    Ok((sentences, res.len()))
}

/// Find sentences by foreign query
async fn foreign_documents(query: &Query) -> Result<SearchResult, error::Error> {
    let mut res = foreign_engine::Find::new(&query.query, query.settings.user_lang, 1000, 0)
        .find()
        .await?;

    if res.len() < 20 && query.settings.show_english {
        res.extend(
            foreign_engine::Find::new(&query.query, Language::English, 1000, 0)
                .find()
                .await?,
        );
    }

    Ok(res)
}

/// Find sentences by native query
async fn japanese_documents(query: &Query) -> Result<SearchResult, error::Error> {
    japanese_engine::Find::new(&query.query, 1000, 0)
        .with_language_filter(query.settings.user_lang)
        .find()
        .await
}
