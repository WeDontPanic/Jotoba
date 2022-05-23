use serde::Deserialize;

use crate::api::app::search::query::deserialize_lang_option;
use crate::jotoba::languages::Language;

#[derive(Deserialize)]
pub struct DetailsPayload {
    pub sequence: u32,
    #[serde(default, deserialize_with = "deserialize_lang_option")]
    pub language: Option<Language>,
    pub show_english: bool,
}
