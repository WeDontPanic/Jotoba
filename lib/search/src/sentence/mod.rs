mod order;
pub mod result;

use std::time::Instant;

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
    let start = Instant::now();

    let lang = query.settings.user_lang;
    let show_english = query.settings.show_english;

    let mut find = Find::new(&query.query, 1000, 0);

    find.with_language_filter(query.settings.user_lang);

    if show_english {
        find.find_engish();
    }

    let res = find.find().await?;

    println!("found {} after: {:?}", res.len(), start.elapsed());

    let sentence = resources::get().sentences();

    let sentences = res
        .retrieve_ordered(|i| sentence.by_id(i as u32))
        .collect::<Vec<_>>();

    let len = sentences.len();

    let sentences = sentences
        .into_iter()
        .filter_map(|i| result::Sentence::from_m_sentence(i.clone(), lang, show_english))
        .map(|i| result::Item { sentence: i })
        .skip(query.page_offset)
        .take(10)
        .collect::<Vec<_>>();

    println!("Sentence search took: {:?}", start.elapsed());

    Ok((sentences, len))
}
