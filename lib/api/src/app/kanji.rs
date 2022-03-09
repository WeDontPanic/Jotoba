use super::{convert_payload, Result};
use actix_web::web::{self, Json};
use types::{
    api::app::{
        query::SearchPayload,
        responses::{
            kanji::{self, CompoundWord},
            paginator::Paginator,
        },
    },
    jotoba::words::Word,
};

/// API response type
pub type Resp = Paginator<kanji::Response>;

/// Do an app kanji search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<Resp>> {
    let query = convert_payload(&payload).parse().unwrap();

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
        .collect::<Vec<kanji::Kanji>>();

    let paginator = Paginator::new(kanji::Response::new(items));

    // TODO: actually make paginator paginate here and cut off items not being part of current page

    Ok(Json(paginator))
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
