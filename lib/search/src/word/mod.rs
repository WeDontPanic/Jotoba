mod kanji;
mod order;
pub mod result;

use std::time::Instant;

use crate::{engine, engine_v2, query::Form};

use self::result::{InflectionInformation, WordResult};
use super::{
    query::{Query, QueryLang},
    search_order::SearchOrder,
};
use error::Error;
use itertools::Itertools;
use japanese::{inflection::SentencePart, JapaneseExt};
use resources::{
    models::{
        kanji::Kanji,
        words::{filter_languages, Word},
    },
    parse::jmdict::part_of_speech::PosSimple,
};
use result::Item;

use japanese::jp_parsing::{InputTextParser, ParseResult, WordItem};
use utils::to_option;

pub(self) struct Search<'a> {
    query: &'a Query,
}

/// Search among all data based on the input query
#[inline]
pub async fn search(query: &Query) -> Result<WordResult, Error> {
    Ok(Search { query }.do_search().await?)
}

#[derive(Default)]
pub(crate) struct ResultData {
    pub(crate) words: Vec<Word>,
    pub(crate) infl_info: Option<InflectionInformation>,
    pub(crate) count: usize,
    pub(crate) sentence_index: i32,
    pub(crate) sentence_parts: Option<Vec<SentencePart>>,
    pub(crate) searched_query: String,
}

impl<'a> Search<'a> {
    /// Do the search
    async fn do_search(&self) -> Result<WordResult, Error> {
        let start = Instant::now();
        let search_result = match self.query.form {
            Form::KanjiReading(_) => kanji::by_reading(self).await?,
            _ => self.do_word_search().await?,
        };

        let words = search_result.words;

        let kanji_results = kanji::load_word_kanji_info(&words)?;

        let res = WordResult {
            contains_kanji: kanji_results.len() > 0,
            items: Self::merge_words_with_kanji(words, kanji_results),
            inflection_info: search_result.infl_info,
            count: search_result.count,
            sentence_parts: search_result.sentence_parts,
            sentence_index: search_result.sentence_index,
            searched_query: search_result.searched_query,
        };
        println!("search took: {:?}", start.elapsed());
        Ok(res)
    }

    /// Search by a word
    async fn do_word_search(&self) -> Result<ResultData, Error> {
        // Perform searches asynchronously
        let (native_word_res, gloss_word_res): (ResultData, ResultData) =
            futures::try_join!(self.native_results(&self.query.query), self.gloss_results())?;

        let sentence_parts = native_word_res
            .sentence_parts
            .map(|i| Some(i))
            .unwrap_or(gloss_word_res.sentence_parts);

        // Chain native and word results into one vector
        Ok(ResultData {
            words: native_word_res
                .words
                .into_iter()
                .chain(gloss_word_res.words)
                .collect_vec(),
            infl_info: native_word_res.infl_info,
            count: native_word_res.count + gloss_word_res.count,
            sentence_parts,
            sentence_index: self.query.word_index as i32,
            searched_query: native_word_res.searched_query,
        })
    }

    async fn get_query<'b>(
        &'b self,
        query_str: &'a str,
    ) -> Result<
        (
            String,
            Option<WordItem<'static, 'b>>,
            Option<Vec<SentencePart>>,
        ),
        Error,
    > {
        if !self.query.parse_japanese {
            return Ok((query_str.to_owned(), None, None));
        }

        let index = engine::word::japanese::index::get();
        let in_db = index.get_indexer().clone().find_term(query_str).is_some();

        let parser = InputTextParser::new(query_str, &japanese::jp_parsing::JA_NL_PARSER, in_db)?;

        if let Some(parsed) = parser.parse() {
            if parsed.items.is_empty() {
                return Ok((query_str.to_owned(), None, None));
            }

            let index = self.query.word_index.clamp(0, parsed.items.len() - 1);
            let res = parsed.items[index].clone();
            let sentence = Self::format_setence_parts(self, parsed).await;

            Ok((res.get_lexeme().to_string(), Some(res), sentence))
        } else {
            Ok((query_str.to_owned(), None, None))
        }
    }

    async fn format_setence_parts(
        &self,
        parsed: ParseResult<'static, 'a>,
    ) -> Option<Vec<SentencePart>> {
        if parsed.items.len() == 1 {
            return None;
        }

        // Lexemes from `parsed` converted to sentence parts
        let mut sentence_parts = parsed
            .items
            .into_iter()
            .enumerate()
            .map(|(pos, i)| i.into_sentence_part(pos as i32))
            .collect_vec();

        // Request furigana for each kanji containing part
        for part in sentence_parts.iter_mut() {
            if !part.text.has_kanji() {
                continue;
            }

            part.furigana = furigana_by_reading(&part.lexeme).await;

            if let Some(ref furigana) = part.furigana {
                let furi_end = match japanese::furigana::last_kana_part(&furigana) {
                    Some(s) => s,
                    None => continue,
                };
                let text_end = match japanese::furigana::last_kana_part(&part.text) {
                    Some(s) => s,
                    None => continue,
                };
                let combined = format!("{}{}", &furigana[..furi_end], &part.text[text_end..]);
                part.furigana = Some(combined)
            }
        }

        Some(sentence_parts)
    }

    /// Perform a native word search
    async fn native_results(&self, query_str: &str) -> Result<ResultData, Error> {
        use engine::word::japanese::Find;

        if self.query.language != QueryLang::Japanese && !query_str.is_japanese() {
            return Ok(ResultData::default());
        }

        let (query, morpheme, sentence) = self.get_query(query_str).await?;

        let query_modified = query != query_str;
        let pos_filter_tags = self
            .query
            .get_part_of_speech_tags()
            .copied()
            .collect::<Vec<_>>();

        let infl_info = morpheme.as_ref().and_then(|i| {
            (!i.inflections.is_empty()).then(|| InflectionInformation {
                lexeme: i.lexeme.to_owned(),
                forms: i.inflections.clone(),
            })
        });

        let mut search_result = Find::new(&query, 1000, 0).find().await?;
        if query_modified {
            let original_res = Find::new(&self.query.query, 1000, 0).find().await?;
            search_result.extend(original_res);
        }

        let pos_filter =
            (!pos_filter_tags.is_empty() && sentence.is_none()).then(|| pos_filter_tags);

        let word_storage = resources::get().words();

        let wordresults = search_result
            .sequence_ids()
            .into_iter()
            .filter_map(|i| word_storage.by_sequence(i))
            .filter(|word| self.word_filter(word, &pos_filter))
            // We're only showing 1000 items max
            .take(1000)
            .collect::<Vec<_>>();

        let count = wordresults.len();

        let mut wordresults: Vec<_> = wordresults
            .into_iter()
            .take(self.query.page_offset + 100)
            .cloned()
            .collect();

        filter_languages(
            wordresults.iter_mut(),
            self.query.settings.user_lang,
            self.query.settings.show_english,
        );

        // Sort the result
        order::new_japanese_order(
            search_result.get_order_map(),
            &SearchOrder::new(self.query, &None),
            &mut wordresults,
        );

        let wordresults: Vec<_> = wordresults
            .into_iter()
            .skip(self.query.page_offset)
            .take(self.query.settings.items_per_page as usize)
            .collect();

        let searched_query = morpheme
            .map(|i| i.original_word.to_owned())
            .unwrap_or(query);

        Ok(ResultData {
            count,
            words: wordresults,
            infl_info,
            sentence_parts: sentence,
            sentence_index: self.query.word_index as i32,
            searched_query,
        })
    }

    /// Search for words by their translations
    async fn gloss_results(&self) -> Result<ResultData, Error> {
        use crate::engine::word::foreign::Find;

        if !matches!(
            self.query.language,
            QueryLang::Foreign | QueryLang::Undetected
        ) {
            return Ok(ResultData::default());
        }

        let pos_filter = to_option(self.query.get_part_of_speech_tags().copied().collect());

        // Do the search
        let search_result = Find::new(&self.query, 1000, 0)
            .with_treshold(0.3)
            .find()
            .await?;

        let word_storage = resources::get().words();

        let wordresults = search_result
            .sequence_ids()
            .into_iter()
            .filter_map(|i| word_storage.by_sequence(i))
            .filter(|word| self.word_filter(word, &pos_filter))
            // We're only showing 1000 items max
            .take(1000)
            .collect::<Vec<_>>();

        // Do romaji search if no results were found
        if wordresults.is_empty() && !self.query.query.is_japanese() {
            return self
                .native_results(&self.query.query.replace(" ", "").to_hiragana())
                .await;
        }

        let count = wordresults.len();

        let mut wordresults = wordresults
            .into_iter()
            .take(self.query.page_offset + 100)
            .cloned()
            .collect::<Vec<_>>();

        filter_languages(
            wordresults.iter_mut(),
            self.query.settings.user_lang,
            self.query.settings.show_english,
        );

        // Sort the result
        order::new_foreign_order(
            search_result.get_order_map(),
            &SearchOrder::new(self.query, &None),
            &mut wordresults,
        );

        let wordresults: Vec<_> = wordresults
            .into_iter()
            .skip(self.query.page_offset)
            .take(self.query.settings.items_per_page as usize)
            .collect();

        Ok(ResultData {
            count,
            words: wordresults,
            ..Default::default()
        })
    }

    #[inline]
    fn merge_words_with_kanji(words: Vec<Word>, kanji: Vec<Kanji>) -> Vec<Item> {
        kanji
            .into_iter()
            .map(|i| i.into())
            .chain(words.into_iter().map(|i| i.into()).collect_vec())
            .collect_vec()
    }

    /// Returns false if a word doesn't match a search-query given filter
    fn word_filter(&self, word: &Word, pos_filter: &Option<Vec<PosSimple>>) -> bool {
        // Apply pos tag filter
        if !pos_filter
            .as_ref()
            .map(|filter| word.has_pos(&filter))
            .unwrap_or(true)
        {
            return false;
        }

        // Apply misc filter
        for misc_filter in self.query.get_misc_tags() {
            if !word.has_misc(*misc_filter) {
                return false;
            }
        }

        true
    }
}

/// Returns furigana of the given `morpheme` if available
async fn furigana_by_reading(morpheme: &str) -> Option<String> {
    use engine::word::japanese::Find;
    let word_storage = resources::get().words();

    let found = Find::new(morpheme, 2, 0).find().await.ok()?;

    let exact_matches = found
        .into_iter()
        .filter(|i| i.relevance > 0.7)
        .filter(|i| {
            let eq = morpheme
                == word_storage
                    .by_sequence(i.seq_id as u32)
                    .unwrap()
                    .get_reading()
                    .reading;
            eq
        })
        .collect::<Vec<_>>();

    // Don't produce potentially wrong furigana if multiple readings are available
    // TODO: guess furigana based on language parser part of speech tag and return it anyways. Let
    // the frontend know that its guessed so it can be previewed differently
    if exact_matches.len() != 1 {
        return None;
    }

    let word = word_storage.by_sequence(exact_matches[0].seq_id as u32)?;
    word.furigana.as_ref().map(|i| i.clone())
}
