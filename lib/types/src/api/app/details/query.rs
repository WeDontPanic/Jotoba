use crate::{
    api::app::deserialize_lang,
    jotoba::language::{LangParam, Language},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DetailsPayload {
    pub sequence: u32,
    #[serde(deserialize_with = "deserialize_lang")]
    pub language: Language,
    pub show_english: bool,
}

impl DetailsPayload {
    #[inline]
    pub fn lang_param(&self) -> LangParam {
        LangParam::with_en_raw(self.language, self.show_english)
    }
}
