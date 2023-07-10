use engine::{
    pushable::FilteredMaxCounter,
    relevance::{data::SortData, RelevanceEngine},
    task::SearchTask,
};
use jp_utils::{
    furi::segment::{AsSegment, SegmentRef},
    JapaneseExt,
};
use ngindex::{item::IndexItem, termset::TermSet};
use sentence_reader::{output::ParseResult, Parser, Part, Sentence};
use types::jotoba::words::{part_of_speech::PosSimple, Word};

use crate::{
    engine::{names, words::native::Engine},
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    word::{
        order::native::NativeOrder,
        result::{InflectionInformation, SentenceInfo},
        Search,
    },
};

use super::task::NativeSearch;

/// Producer for sentence reader and inflection information
pub struct SReaderProducer<'a> {
    query: &'a Query,
    parsed: ParseResult,
}

impl<'a> SReaderProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        let parsed = Parser::new(&query.query_str).parse();
        Self { query, parsed }
    }

    /// Search task for inflected word
    fn infl_task(&self) -> Option<SearchTask<'static, Engine>> {
        let infl = self.parsed.as_inflected_word()?;

        let normalized = infl.get_normalized();

        let original_query = <Engine as engine::Engine>::make_query(&self.query.query_str, None)?;

        let search = NativeSearch::new(self.query, &normalized);
        let o_query = search.original_query().to_string();
        let order = NativeOrder::new(o_query).with_oquery_ts(original_query);
        Some(search.task().with_custom_order(order))
    }

    /// Selected word index within the sentence
    #[inline]
    fn sentence_index(&self) -> usize {
        self.parsed
            .as_sentence()
            .map(|s| self.query.word_index.clamp(0, s.word_count() - 1))
            .unwrap_or(0)
    }

    /// Selected word in the sentence
    #[inline]
    fn sentence_word(&self) -> Option<&Part> {
        let sentence = self.parsed.as_sentence()?;
        let index = self.sentence_index();
        sentence.get_at(index)
    }

    /// Normalized search task for sentences
    fn snt_task_normalized(&self) -> Option<SearchTask<'static, Engine>> {
        let word = self.sentence_word().unwrap();

        let inflected = word.get_inflected();
        let normalized = word.get_normalized();

        let search = NativeSearch::new(self.query, &normalized);

        let order = NativeOrder::new(inflected).with_w_index(self.sentence_index());

        Some(search.task().with_custom_order(order))
    }

    /// Inflected search task for an inflected word in a sentence
    fn snt_task_infl(&self) -> Option<SearchTask<'static, Engine>> {
        let word = self.sentence_word().unwrap();
        let inflected = word.get_inflected();
        let search = NativeSearch::new(self.query, &inflected);
        let o_query = search.original_query().to_string();
        let order = NativeOrder::new(o_query).with_w_index(self.sentence_index());
        Some(search.task().with_custom_order(order))
    }
}

impl<'a> Producer for SReaderProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        if let ParseResult::InflectedWord(infl) = &self.parsed {
            self.infl_task().unwrap().find_to(out);
            out.output_add.inflection = InflectionInformation::from_part(infl);
            return;
        }

        if let ParseResult::Sentence(mut sentence) = self.parsed.clone() {
            set_furigana(&mut sentence);

            self.snt_task_normalized().unwrap().find_to(out);

            let word = self.sentence_word().unwrap();
            if word.get_inflected() != word.get_normalized() {
                self.snt_task_infl().unwrap().find_to(out);
            }

            out.output_add.inflection = InflectionInformation::from_part(word);
            out.output_add.raw_query = word.get_inflected();
            out.output_add.sentence = Some(SentenceInfo {
                parts: Some(sentence.clone()),
                index: self.query.word_index,
                query: word.get_normalized(),
            });
        }
    }

    fn should_run(&self, already_found: usize) -> bool {
        if self.parsed.is_none()
            || self.query.q_lang != QueryLang::Japanese
            || !self.query.form.is_normal()
            || self.query.query_str.is_empty()
        {
            return false;
        }

        // Always run inlfections
        if self.parsed.is_inflected_word() {
            return true;
        }

        // Disable sentence reader if already found some words
        if already_found > 0 {
            return false;
        }

        let term_in_db = word_exists(&self.query.query_str);
        // For sentences only run if the query is not a term in the db
        !term_in_db
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        if let Some(mut infl) = self.infl_task() {
            infl.estimate_to(out);
            return;
        }

        if self.parsed.is_sentence() {
            self.snt_task_normalized().unwrap().estimate_to(out);
            let word = self.sentence_word().unwrap();
            if word.get_inflected() != word.get_normalized() {
                self.snt_task_infl().unwrap().estimate_to(out);
            }
        }
    }
}

/// Returns `true` if the word exists in all words
fn word_exists(term: &str) -> bool {
    let task = SearchTask::<Engine>::new(term).with_limit(1);

    let query = term.to_string();
    let mut task = task.with_item_filter(move |i| {
        resources::get()
            .words()
            .by_sequence(*i.item())
            .unwrap()
            .has_reading(&query)
    });

    let res = task.find();
    res.len() > 0
}

/// Generates furigana for a sentence
fn set_furigana(s: &mut Sentence) {
    for part in s.iter_mut() {
        let p = part.clone();
        part.set_furigana(|inp| furigana_by_reading(inp, &p))
    }
}

/// Returns furigana of the given `morpheme` if available
fn furigana_by_reading(morpheme: &str, part: &sentence_reader::Part) -> Option<String> {
    word_furi(morpheme, part).or_else(|| name_furi(morpheme))
}

fn name_furi(morpheme: &str) -> Option<String> {
    let morpheme_c = morpheme.to_string();

    let mut task = SearchTask::<names::native::Engine>::new(morpheme)
        .with_limit(1)
        .with_result_filter(move |n| n.get_reading() == morpheme_c && n.has_kanji());

    let res = task.find();

    if res.total_items != 1 {
        return None;
    }

    let name = res.get(0).unwrap().item;
    let kanji = name.kanji.as_ref().unwrap();
    Some(SegmentRef::new_kanji(&kanji, &[&name.kana]).encode())
}

fn word_furi(morpheme: &str, part: &sentence_reader::Part) -> Option<String> {
    let word_storage = resources::get().words();

    let pos = sentence_reader::part::wc_to_simple_pos(&part.word_class_raw());
    let morph = morpheme.to_string();

    let mut st = SearchTask::<Engine>::new(morpheme)
        .with_limit(10)
        .with_custom_order(WordFuriOrder::new(pos, morpheme.to_string()))
        .with_result_filter(move |i| i.has_reading(&morph));

    st.find().get(0).and_then(|word| {
        word_storage
            .by_sequence(word.item.sequence)
            .and_then(|i| i.furigana.clone())
    })
}

struct WordFuriOrder {
    pos: Option<PosSimple>,
    morph: String,
}

impl WordFuriOrder {
    #[inline]
    fn new(pos: Option<PosSimple>, morph: String) -> Self {
        Self { pos, morph }
    }
}

impl RelevanceEngine for WordFuriOrder {
    type OutItem = &'static Word;
    type IndexItem = IndexItem<u32>;
    type Query = TermSet;

    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        let mut score = 0.0;

        let i = item.item();
        let reading = &i.get_reading().reading;
        let reading_len = utils::real_string_len(reading);

        if reading == &self.morph {
            score += 100.0;
        }

        if reading_len == 1 && reading.is_kanji() {
            let kanji = reading.chars().next().unwrap();
            let kana = i.get_kana();
            let norm = indexes::get()
                .kanji()
                .reading_freq()
                .norm_reading_freq(kanji, kana);
            if let Some(norm) = norm {
                score += norm * 10.0;
            }
        }

        if let Some(ref pos) = self.pos {
            if i.has_pos(&[*pos]) {
                score += 20.0;
            } else {
                //score = score.saturating_sub(30);
                score = (score - 30.0).max(0.0);
            }
        }

        if i.is_common() {
            score += 2.0;
        }

        if i.get_jlpt_lvl().is_some() {
            score += 2.0;
        }

        score
    }
}
