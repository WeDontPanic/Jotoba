mod kanji;
pub mod order;
pub mod result;
pub mod tag_only;

use std::time::Instant;

use crate::{
    engine::{
        guess::Guess,
        words::{foreign, native},
        SearchTask,
    },
    query::Form,
};

use self::result::{InflectionInformation, WordResult};
use super::query::{Query, QueryLang};
use error::Error;
use itertools::Itertools;
use japanese::{inflection::SentencePart, JapaneseExt};
use resources::{
    models::{
        kanji::Kanji,
        words::{filter_languages, Word},
    },
    parse::jmdict::{languages::Language, part_of_speech::PosSimple},
};
use result::Item;

use japanese::jp_parsing::{InputTextParser, ParseResult, WordItem};
use utils::to_option;

pub(self) struct Search<'a> {
    query: &'a Query,
}

/// Search among all data based on the input query
#[inline]
pub fn search(query: &Query) -> Result<WordResult, Error> {
    Ok(Search { query }.do_search()?)
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
    fn do_search(&self) -> Result<WordResult, Error> {
        let start = Instant::now();
        let search_result = match self.query.form {
            Form::KanjiReading(_) => kanji::by_reading(self)?,
            Form::TagOnly => tag_only::search(self)?,
            _ => self.do_word_search()?,
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
    fn do_word_search(&self) -> Result<ResultData, Error> {
        let native_word_res = self.native_results(&self.query.query)?;
        let gloss_word_res = self.gloss_results()?;

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

    fn get_query<'b>(
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

        let in_db = SearchTask::<native::Engine>::new(query_str).has_term();

        let parser = InputTextParser::new(query_str, &japanese::jp_parsing::JA_NL_PARSER, in_db)?;

        if let Some(parsed) = parser.parse() {
            if parsed.items.is_empty() {
                return Ok((query_str.to_owned(), None, None));
            }

            let index = self.query.word_index.clamp(0, parsed.items.len() - 1);
            let res = parsed.items[index].clone();
            let sentence = Self::format_setence_parts(self, parsed);

            Ok((res.get_lexeme().to_string(), Some(res), sentence))
        } else {
            Ok((query_str.to_owned(), None, None))
        }
    }

    fn format_setence_parts(&self, parsed: ParseResult<'static, 'a>) -> Option<Vec<SentencePart>> {
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

            part.furigana = furigana_by_reading(&part.lexeme);

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

    /// Returns a `SearchTask` for the current query. This will be used to find all words for
    /// the search
    fn native_search_task<'b>(
        &self,
        query: &'b str,
        original_query: &str,
        sentence: bool,
    ) -> SearchTask<'b, native::Engine> {
        let mut search_task: SearchTask<native::Engine> = SearchTask::new(&query)
            .limit(self.query.settings.items_per_page as usize)
            .offset(self.query.page_offset)
            .threshold(0.04f32);

        // apply user filter
        let q_cloned = self.query.clone();
        let pos_filter = self.get_pos_filter(sentence);
        search_task.set_result_filter(move |word| Self::word_filter(&q_cloned, word, &pos_filter));

        // Set order function;
        let original_query = original_query.to_string();
        search_task.set_order_fn(move |word, rel, q_str, _| {
            order::japanese_search_order(word, rel, q_str, Some(&original_query))
        });

        search_task
    }

    /// Perform a native word search
    fn native_results(&self, query_str: &str) -> Result<ResultData, Error> {
        if self.query.language != QueryLang::Japanese && !query_str.is_japanese() {
            return Ok(ResultData::default());
        }

        let (query, morpheme, sentence) = self.get_query(query_str)?;

        let mut search_task =
            self.native_search_task(&query, &self.query.query, sentence.is_some());

        // If query was modified (ie. through reflection), search for original too
        if query != query_str {
            search_task.add_query(&self.query.query);
        }

        let res = search_task.find()?;
        let count = res.len();

        let mut wordresults = res.item_iter().cloned().collect::<Vec<_>>();

        filter_languages(
            wordresults.iter_mut(),
            self.query.settings.user_lang,
            self.query.settings.show_english,
        );

        let infl_info = inflection_info(&morpheme);

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

    /// Returns a `SearchTask` for the current query. This will be used to find all words for
    /// the search
    fn gloss_search_task(&self) -> SearchTask<foreign::Engine> {
        let mut search_task: SearchTask<foreign::Engine> =
            SearchTask::with_language(&self.query.query, self.query.settings.user_lang)
                .limit(self.query.settings.items_per_page as usize)
                .offset(self.query.page_offset)
                .threshold(0.3f32);

        if self.query.settings.show_english && self.query.settings.user_lang != Language::English {
            search_task.add_language_query(&self.query.query, Language::English);
        }

        // Set user defined filter
        let pos_filter = to_option(self.query.get_part_of_speech_tags().copied().collect());
        let q_cloned = self.query.clone();
        search_task.set_result_filter(move |word| Self::word_filter(&q_cloned, word, &pos_filter));

        // Set order function
        let user_lang = self.query.settings.user_lang;
        search_task.set_order_fn(move |word, relevance, query, language| {
            order::foreign_search_order(word, relevance, query, language.unwrap(), user_lang)
        });

        search_task
    }

    /// Search for words by their translations
    fn gloss_results(&self) -> Result<ResultData, Error> {
        if !matches!(
            self.query.language,
            QueryLang::Foreign | QueryLang::Undetected
        ) {
            return Ok(ResultData::default());
        }

        let search_task = self.gloss_search_task();

        // Do the search
        let res = search_task.find()?;
        let count = res.len();

        let mut wordresults = res.item_iter().cloned().collect::<Vec<_>>();

        // Do romaji search if no results were found
        if wordresults.is_empty() && !self.query.query.is_japanese() {
            return self.native_results(&self.query.query.replace(" ", "").to_hiragana());
        }

        filter_languages(
            wordresults.iter_mut(),
            self.query.settings.user_lang,
            self.query.settings.show_english,
        );

        Ok(ResultData {
            count,
            words: wordresults,
            ..Default::default()
        })
    }

    fn get_pos_filter(&self, is_sentence: bool) -> Option<Vec<PosSimple>> {
        let pos_filter_tags = self
            .query
            .get_part_of_speech_tags()
            .copied()
            .collect::<Vec<_>>();

        (!pos_filter_tags.is_empty() && !is_sentence).then(|| pos_filter_tags)
    }

    #[inline]
    fn merge_words_with_kanji(words: Vec<Word>, kanji: Vec<Kanji>) -> Vec<Item> {
        kanji
            .into_iter()
            .map(|i| i.into())
            .chain(words.into_iter().map(|i| i.into()).collect_vec())
            .collect_vec()
    }

    /// Returns false if a word should be filtered out of results
    fn word_filter(query: &Query, word: &Word, pos_filter: &Option<Vec<PosSimple>>) -> bool {
        // Apply pos tag filter
        if !pos_filter
            .as_ref()
            .map(|filter| word.has_pos(&filter))
            .unwrap_or(true)
        {
            return false;
        }

        if !word.has_language(query.settings.user_lang, query.settings.show_english) {
            return false;
        }

        // Apply misc filter
        for misc_filter in query.get_misc_tags() {
            if !word.has_misc(*misc_filter) {
                return false;
            }
        }

        true
    }
}

/// Returns information about word inflections, if available
fn inflection_info(morpheme: &Option<WordItem>) -> Option<InflectionInformation> {
    morpheme.as_ref().and_then(|i| {
        (!i.inflections.is_empty()).then(|| InflectionInformation {
            lexeme: i.lexeme.to_owned(),
            forms: i.inflections.clone(),
        })
    })
}

/// Returns furigana of the given `morpheme` if available
fn furigana_by_reading(morpheme: &str) -> Option<String> {
    let word_storage = resources::get().words();

    let st = SearchTask::<native::Engine>::new(morpheme)
        .threshold(0.7)
        .limit(2);

    let found = st.find().ok()?;

    // Don't produce potentially wrong furigana if multiple readings are available
    // TODO: guess furigana based on language parser part of speech tag and return it anyways. Let
    // the frontend know that its guessed so it can be previewed differently
    if found.len() != 1 {
        return None;
    }

    let word = word_storage.by_sequence(found[0].item.sequence as u32)?;
    word.furigana.as_ref().map(|i| i.clone())
}

/// Guesses the amount of results a search would return with given `query`
pub fn guess_result(query: &Query) -> Option<Guess> {
    let search = Search { query };

    if query.language == QueryLang::Japanese {
        guess_native(search)
    } else {
        guess_foreign(search)
    }
}

fn guess_native(search: Search) -> Option<Guess> {
    let (query, _, sentence) = search.get_query(&search.query.query).ok()?;

    search
        .native_search_task(&query, &search.query.query, sentence.is_some())
        .estimate_result_count()
        .ok()
}

fn guess_foreign(search: Search) -> Option<Guess> {
    search.gloss_search_task().estimate_result_count().ok()
}
