use crate::app::Result;
use actix_web::web::Json;
use error::api_error::RestError;
use japanese::JapaneseExt;
use types::{
    api::app::{
        details::{
            query::DetailsPayload,
            word::{self, TransitivityPair},
        },
        search::responses::{kanji::Kanji, words::Word},
    },
    jotoba::{languages::Language, words::adjust_language},
};

pub async fn details(payload: Json<DetailsPayload>) -> Result<Json<word::Details>> {
    Ok(Json(
        Details::new(&payload)
            .ok_or(RestError::NotFound)?
            .get_details(),
    ))
}

pub(crate) struct Details<'a> {
    payload: &'a DetailsPayload,
    word: &'static types::jotoba::words::Word,
}

impl<'a> Details<'a> {
    #[inline]
    fn new(payload: &'a DetailsPayload) -> Option<Self> {
        let word = resources::get().words().by_sequence(payload.sequence)?;
        Some(Details { payload, word })
    }

    fn get_details(&self) -> word::Details {
        let kanji = self.get_kanji();
        let has_sentence = self.has_sentence();
        let transitivity_pair = self.transitivity_pair();
        let collocations = self.get_collocations();
        let inflection_table = self.word.get_inflections();

        let word = self.get_word();

        word::Details::new(
            word,
            kanji,
            inflection_table,
            collocations,
            has_sentence,
            transitivity_pair,
        )
    }

    fn get_kanji(&self) -> Vec<Kanji> {
        let retrieve = resources::get().kanji();

        self.word
            .get_reading()
            .reading
            .chars()
            .filter_map(|i| i.is_kanji().then(|| i).and_then(|k| retrieve.by_literal(k)))
            .map(|i| (*i).clone().into())
            .collect::<Vec<_>>()
    }

    #[inline]
    fn has_sentence(&self) -> bool {
        self.word.has_sentence(self.payload.language)
            || (self.payload.show_english && self.word.has_sentence(Language::English))
    }

    fn transitivity_pair(&self) -> Option<TransitivityPair> {
        if let Some(trans) = self.word.transive_verion {
            return Some(TransitivityPair::Transitive(trans));
        }

        if let Some(intrans) = self.word.intransive_verion {
            return Some(TransitivityPair::Intransitive(intrans));
        }

        None
    }

    fn get_collocations(&self) -> Vec<Word> {
        let collocations = match &self.word.collocations {
            Some(colloc) => colloc,
            None => return vec![],
        };
        let retrieve = resources::get().words();

        collocations
            .iter()
            .filter_map(|i| {
                let word = retrieve.by_sequence(*i)?;
                Some(self.format_word(word))
            })
            .collect()
    }

    #[inline]
    fn get_word(&self) -> Word {
        self.format_word(self.word)
    }

    #[inline]
    fn format_word(&self, word: &types::jotoba::words::Word) -> Word {
        let mut word = word.clone();
        adjust_language(&mut word, self.payload.language, self.payload.show_english);

        crate::app::conv_word(word, self.payload.language)
    }
}
