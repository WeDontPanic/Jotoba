use sentence_reader::{output::ParseResult, Parser};

use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    word::{result::InflectionInformation, Search},
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
            println!("{infl:#?}");
            println!("{}", infl.get_normalized());
            let normalized = infl.get_normalized();
            NativeSearch::new(self.query, &normalized)
                .task()
                .find_to(out);
            out.output_add.inflection = InflectionInformation::from_part(infl);
            return;
        }

        if let ParseResult::Sentence(sentence) = &self.parsed {
            let word = match sentence.get_at(self.query.word_index) {
                Some(w) => w,
                None => return,
            };

            println!("{word:#?}");

            // Find normalized
            NativeSearch::new(self.query, &word.get_normalized())
                .task()
                .find_to(out);

            // Find inflected
            NativeSearch::new(self.query, &word.get_inflected())
                .task()
                .find_to(out);
        }

        /*
        NativeSearch::new(self.query, &self.query.query_str)
            .task()
            .find_to(out);
        */
    }

    fn should_run(&self, _already_found: usize) -> bool {
        !self.parsed.is_none()
    }
}
