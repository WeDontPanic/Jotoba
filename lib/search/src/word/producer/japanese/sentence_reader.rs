use japanese::JapaneseExt;
use sentence_reader::{output::ParseResult, Parser, Part, Sentence};
use types::jotoba::words::{part_of_speech::PosSimple, Word};

use crate::{
    engine::{search_task::cpushable::FilteredMaxCounter, words::native, SearchTask},
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    word::{
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
    fn infl_task(&self) -> Option<SearchTask<native::Engine>> {
        let infl = self.parsed.as_inflected_word()?;
        let normalized = infl.get_normalized();
        Some(NativeSearch::new(self.query, &normalized).task())
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
    fn snt_task_normalized(&self) -> Option<SearchTask<native::Engine>> {
        let word = self.sentence_word().unwrap();
        Some(NativeSearch::new(self.query, &word.get_normalized()).task())
    }

    /// Inflected search task for an inflected word in a sentence
    fn snt_task_infl(&self) -> Option<SearchTask<native::Engine>> {
        let word = self.sentence_word().unwrap();
        Some(NativeSearch::new(self.query, &word.get_inflected()).task())
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

    fn should_run(&self, _already_found: usize) -> bool {
        if self.parsed.is_none()
            || self.query.q_lang != QueryLang::Japanese
            || !self.query.form.is_normal()
            || self.query.is_regex()
            || self.query.query_str.is_empty()
        {
            return false;
        }

        // Always run inlfections
        if self.parsed.is_inflected_word() {
            return true;
        }

        // Only run sentence reader search if the query is not a term in the index
        !NativeSearch::has_term(&self.query.query_str)
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        if let Some(infl) = self.infl_task() {
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

/// Generates furigana for a sentence
fn set_furigana(s: &mut Sentence) {
    for part in s.iter_mut() {
        let p = part.clone();
        part.set_furigana(|inp| furigana_by_reading(inp, &p))
    }
}

/// Returns furigana of the given `morpheme` if available
fn furigana_by_reading(morpheme: &str, part: &sentence_reader::Part) -> Option<String> {
    let word_storage = resources::get().words();

    let mut st = SearchTask::<native::Engine>::new(morpheme).limit(10);

    let pos = sentence_reader::part::wc_to_simple_pos(&part.word_class_raw());
    let morph = morpheme.to_string();
    st.with_custom_order(move |item| furi_order(item.item(), &pos, &morph));

    let morph = morpheme.to_string();
    st.set_result_filter(move |i| i.has_reading(&morph));

    let found = st.find();
    println!("res: {found:#?}");
    word_storage
        .by_sequence(found.get(0)?.item.sequence)?
        .furigana
        .clone()
}

fn furi_order(i: &Word, pos: &Option<PosSimple>, morph: &str) -> usize {
    let mut score: usize = 0;

    let reading = &i.get_reading().reading;
    let reading_len = utils::real_string_len(reading);

    if reading == morph {
        score += 100;
    }

    if reading_len == 1 && reading.is_kanji() {
        let kanji = reading.chars().next().unwrap();
        let kana = i.get_kana();
        let norm = indexes::get()
            .kanji()
            .reading_fre()
            .norm_reading_freq(kanji, kana);
        if let Some(norm) = norm {
            score += (norm * 10.0) as usize;
        }
    }

    if let Some(pos) = pos {
        if i.has_pos(&[*pos]) {
            score += 20;
        } else {
            score = score.saturating_sub(30);
        }
    }

    if i.is_common() {
        score += 2;
    }

    if i.get_jlpt_lvl().is_some() {
        score += 2;
    }

    score
}
