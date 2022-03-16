use crate::app::new_page;

use super::{convert_payload, Result};
use actix_web::web::{self, Json};
use error::api_error::RestError;
use types::{
    api::app::{
        query::SearchPayload,
        responses::kanji::{self, CompoundWord},
    },
    jotoba::{pagination::page::Page, words::Word},
};

/// API response type
pub type Resp = Page<kanji::Response>;

/// Do an app kanji search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<Resp>> {
    let query = convert_payload(&payload)
        .parse()
        .ok_or(RestError::BadRequest)?;

    let result = web::block(move || search::kanji::search(&query)).await??;

    let items = result
        .items
        .into_iter()
        .map(|i| {
            let mut k: kanji::Kanji = i.kanji.into();
            k.set_on_compounds(convert_dicts(&i.on_dicts));
            k.set_kun_compounds(convert_dicts(&i.kun_dicts));
            k
        })
        .collect::<Vec<_>>();

    let len = result.total_items as u32;
    let kanji = kanji::Response::new(items);
    let page = new_page(&payload, kanji, len, payload.settings.kanji_page_size);

    Ok(Json(page))
}

#[inline]
fn convert_dicts(dicts: &Option<Vec<Word>>) -> Vec<CompoundWord> {
    dicts
        .as_ref()
        .map(|i| {
            i.iter()
                .map(|j| CompoundWord::from_word(&j))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}
