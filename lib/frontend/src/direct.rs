use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use config::Config;
use localization::TranslationDict;
use search::{
    query::{Query, UserSettings},
    sentence::{self, result::SentenceResult},
    word::{self, result::WordResult},
};
use types::jotoba::{search::QueryType, words::filter_languages};

use crate::{
    og_tags::{self, TagKeyName},
    search_ep::redirect_home,
    templates, user_settings,
    web_error::{self, Error},
    BaseData, ResultData, SearchResult,
};

/// Endpoint to perform a search
pub async fn direct_ep(
    h_query: web::Path<(u8, String)>,
    locale_dict: web::Data<Arc<TranslationDict>>,
    config: web::Data<Config>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let settings = user_settings::parse(&request);

    let (stype, id) = h_query.into_inner();
    let query_type = QueryType::try_from(stype).map_err(|_| Error::BadRequest)?;

    let result_data = match query_type {
        QueryType::Words => find_direct_word(&id, &settings).await,
        QueryType::Names => find_direct_name(&id).await,
        QueryType::Sentences => find_direct_sentence(&id, &settings).await,
        QueryType::Kanji => return Ok(redirect_home()),
    };

    if let Err(err) = result_data {
        return match err {
            web_error::Error::NotFound => Err(err),
            _ => Ok(redirect_home()),
        };
    }

    let query = Query::default();
    let mut base_data = BaseData::new(&locale_dict, settings, &config.asset_hash, &config)
        .with_search_result(&query, result_data.unwrap(), None);

    set_og_tag(&mut base_data, query_type);

    Ok(HttpResponse::Ok().body(render!(templates::base, base_data).render()))
}

fn set_og_tag(base_data: &mut BaseData, query_type: QueryType) {
    let search_result = base_data.site.as_search_result().unwrap();
    let mut search_res_og = og_tags::TagSet::with_capacity(5);

    let title = match query_type {
        QueryType::Kanji => return,
        QueryType::Sentences => "Jotoba sentence".to_string(),
        QueryType::Names => format!("{} - Jotoba name", search_res_val(&search_result).unwrap()),
        QueryType::Words => format!("{} - Jotoba word", search_res_val(&search_result).unwrap()),
    };

    let descrption = "Jotoba entry. See more...";

    search_res_og.add_og(TagKeyName::Title, &title);
    search_res_og.add_twitter(TagKeyName::Title, &title);
    search_res_og.add_og(TagKeyName::Description, descrption);
    search_res_og.add_twitter(TagKeyName::Description, descrption);
    search_res_og.add_twitter(TagKeyName::Card, "summary");

    base_data.set_og_tags(search_res_og);
}

fn search_res_val(res: &SearchResult) -> Option<String> {
    Some(match &res.result {
        ResultData::Word(w) => w.get_items().0[0].get_reading().reading.clone(),
        ResultData::Name(n) => n[0].kanji.as_ref().unwrap_or(&n[0].kana).to_string(),
        _ => return None,
    })
}

/// Find direct word
pub async fn find_direct_word(id: &str, settings: &UserSettings) -> Result<ResultData, Error> {
    let sequence_id: u32 = id.parse().map_err(|_| Error::NotFound)?;

    let res_name = resources::get()
        .words()
        .by_sequence(sequence_id)
        .ok_or(web_error::Error::NotFound)?
        .clone();

    let mut results = vec![res_name];

    // also show enlgish if otherwise no results would be shown due users settings
    let show_english = !results[0].has_language(settings.user_lang, false) || settings.show_english;
    filter_languages(results.iter_mut(), settings.user_lang, show_english);

    let kanji = search::word::kanji::load_word_kanji_info(&results)
        .into_iter()
        .map(|k| word::result::Item::Kanji(k));

    let word = results.remove(0);

    let mut items = vec![word::result::Item::Word(word)];
    items.extend(kanji);
    let contains_kanji = items.len() > 1;

    Ok(ResultData::Word(WordResult {
        items,
        count: 1,
        contains_kanji,
        inflection_info: None,
        sentence_parts: None,
        sentence_index: 0,
        searched_query: String::new(),
    }))
}

/// Find direct name
pub async fn find_direct_name(id: &str) -> Result<ResultData, Error> {
    let sequence_id: u32 = id.parse().map_err(|_| Error::NotFound)?;

    let res_word = resources::get()
        .names()
        .by_sequence(sequence_id)
        .ok_or(web_error::Error::NotFound)?;

    Ok(ResultData::Name(vec![res_word]))
}

/// Find direct sentence
pub async fn find_direct_sentence(id: &str, settings: &UserSettings) -> Result<ResultData, Error> {
    let sequence_id: u32 = id.parse().map_err(|_| Error::NotFound)?;

    let res_sentence = resources::get()
        .sentences()
        .by_id(sequence_id)
        .ok_or(web_error::Error::NotFound)?
        .clone();

    let res_sentence =
        sentence::result::Sentence::from_m_sentence(res_sentence, settings.user_lang, true)
            .unwrap();

    Ok(ResultData::Sentence(SentenceResult {
        items: vec![sentence::result::Item {
            sentence: res_sentence,
        }],
        len: 1,
        hidden: false,
    }))
}
