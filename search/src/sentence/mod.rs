use error::Error;
use models::DbPool;
use parse::jmdict::languages::Language;
use sentencesearch::SentenceSearch;

use self::result::{Item, Sentence};

use super::query::{Query, QueryLang};
use itertools::Itertools;

mod order;
pub mod result;
mod sentencesearch;

/// Searches for sentences
pub async fn search(db: &DbPool, query: &Query) -> Result<Vec<result::Item>, Error> {
    if query.language == QueryLang::Japanese {
        search_jp(db, query).await
    } else {
        search_foreign(db, query).await
    }
}

/// Searches for sentences (jp input)
pub async fn search_jp(db: &DbPool, query: &Query) -> Result<Vec<result::Item>, Error> {
    let search = SentenceSearch::new(db, &query.query, query.settings.user_lang);
    let sentences = search.by_jp().await?;

    Ok(merge_results(sentences, query.settings.user_lang))
}

/// Searches for sentences (other input)
pub async fn search_foreign(db: &DbPool, query: &Query) -> Result<Vec<result::Item>, Error> {
    let search = SentenceSearch::new(db, &query.query, query.settings.user_lang);
    let sentences = search.by_foreign().await?;
    Ok(merge_results(sentences, query.settings.user_lang))
}

fn merge_results(results: Vec<Sentence>, user_lang: Language) -> Vec<Item> {
    results
        .into_iter()
        .group_by(|i| i.id)
        .into_iter()
        .filter_map(|(_, i)| {
            let mut sentence = i.into_iter().next()?;

            if user_lang == Language::English {
                sentence.eng = String::from("-");
            }

            Some(Item { sentence })
        })
        .collect_vec()
}
