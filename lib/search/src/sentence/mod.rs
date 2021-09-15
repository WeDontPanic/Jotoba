mod order;
pub mod result;

use super::query::Query;
use crate::query::QueryLang;
use error::Error;

/// Searches for sentences
pub async fn search(query: &Query) -> Result<(Vec<result::Item>, usize), Error> {
    if query.language == QueryLang::Japanese {
        search_jp(query).await
    } else {
        // search_foreign(db, query).await
        unimplemented!()
    }
}

/// Searches for sentences (jp input)
pub async fn search_jp(query: &Query) -> Result<(Vec<result::Item>, usize), Error> {
    use crate::engine::sentences::japanese::Find;

    let res = Find::new(&query.query, 10000, 0).find().await?;

    let sentence = resources::get().sentences();

    let sentences = res
        .retrieve_ordered(|i| {
            let sentence = sentence.by_id(i as u32)?;
            sentence
                .has_translation(query.settings.user_lang)
                .then(|| sentence)
        })
        .collect::<Vec<_>>();

    let len = sentences.len();

    let sentences = sentences
        .into_iter()
        .filter_map(|i| result::Sentence::from_m_sentence(i.clone(), query.settings.user_lang))
        .map(|i| result::Item { sentence: i })
        .skip(query.page_offset)
        .take(10)
        .collect::<Vec<_>>();

    Ok((sentences, len))
}
