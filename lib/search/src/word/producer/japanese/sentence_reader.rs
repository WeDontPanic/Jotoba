use sentence_reader::{output::ParseResult, Parser, Sentence};
use types::jotoba::words::{part_of_speech::PosSimple, Word};

use crate::{
    engine::{words::native, SearchTask},
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    word::{
        result::{InflectionInformation, SentenceInfo},
        Search,
    },
};

use super::task::NativeSearch;

/// Produces search results for native search input
pub struct SReaderProducer<'a> {
    query: &'a Query,
    parsed: ParseResult,
}

impl<'a> SReaderProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        let parsed = Parser::new(&query.query_str).parse();
        Self { query, parsed }
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
            let normalized = infl.get_normalized();
            NativeSearch::new(self.query, &normalized)
                .task()
                .find_to(out);
            out.output_add.inflection = InflectionInformation::from_part(infl);
            return;
        }

        if let ParseResult::Sentence(mut sentence) = self.parsed.clone() {
            set_furigana(&mut sentence);

            let index = self.query.word_index.clamp(0, sentence.word_count() - 1);
            let word = sentence.get_at(index).unwrap();

            // Find normalized
            NativeSearch::new(self.query, &word.get_normalized())
                .task()
                .find_to(out);

            if word.get_inflected() != word.get_normalized() {
                // Find inflected
                NativeSearch::new(self.query, &word.get_inflected())
                    .task()
                    .find_to(out);
            }

            out.output_add.inflection = InflectionInformation::from_part(word);
            out.output_add.raw_query = word.get_normalized();
            out.output_add.sentence = Some(SentenceInfo {
                parts: Some(sentence.clone()),
                index: self.query.word_index,
                query: word.get_normalized(),
            });
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        if self.query.as_regex_query().is_some()
            || self.parsed.is_none()
            || !self.query.form.is_normal()
        {
            return false;
        }

        if self.parsed.is_inflected_word() {
            return true;
        }

        !NativeSearch::new(self.query, &self.query.query_str)
            .task()
            .has_term()
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

    let mut st = SearchTask::<native::Engine>::new(morpheme)
        .threshold(0.7)
        .limit(10);

    let pos = sentence_reader::part::wc_to_simple_pos(&part.word_class_raw());
    let morph = morpheme.to_string();
    st.with_custom_order(move |item| furi_order(item.item(), &pos, &morph));

    let found = st.find();
    if found.is_empty() {
        return None;
    }
    let word = word_storage.by_sequence(found[0].item.sequence as u32)?;
    word.furigana.as_ref().cloned()
}

fn furi_order(i: &Word, pos: &Option<PosSimple>, morph: &str) -> usize {
    let mut score: usize = 100;
    if i.get_reading().reading != morph {
        score = 0;
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
