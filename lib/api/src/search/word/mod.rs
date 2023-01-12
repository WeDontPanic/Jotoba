use super::{Result, SearchRequest};
use actix_web::web::{self, Data, Json};
use config::Config;
use search::{word::Search, SearchExecutor};
use types::{
    api::search::{
        kanji::Kanji,
        word::{Response, Word},
    },
    jotoba::search::SearchTarget,
};

/// Do a word search via API
pub async fn word_search(
    payload: Json<SearchRequest>,
    config: Data<Config>,
) -> Result<Json<Response>> {
    let query = super::parse_query(payload, SearchTarget::Words)?;
    let result = web::block(move || {
        let search = Search::new(&query);
        SearchExecutor::new(search).run()
    })
    .await?;

    let kanji: Vec<Kanji> = search::word::kanji::load_word_kanji_info(&result.items)
        .into_iter()
        .map(|i| Kanji::from(&i, config.server.get_html_files()))
        .collect();
    let words: Vec<Word> = result.items.into_iter().map(|i| (&i).into()).collect();
    Ok(Json(Response::new(words, kanji)))
}
