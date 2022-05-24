use crate::{api::app::deserialize_lang, jotoba::languages::Language};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DetailsPayload {
    pub sequence: u32,
    #[serde(deserialize_with = "deserialize_lang")]
    pub language: Language,
    pub show_english: bool,
}
