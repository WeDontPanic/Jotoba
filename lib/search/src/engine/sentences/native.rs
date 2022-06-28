use std::collections::HashSet;

use crate::engine::{Indexable, SearchEngine};
use indexes::sentences::document::SentenceDocument;
use resources::storage::ResourceStorage;
use sentence_reader::output::ParseResult;
use types::jotoba::{languages::Language, sentences::Sentence};
use vector_space_model2::{build::weights::TFIDF, DefaultMetadata, Vector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = SentenceDocument;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model2::Index<Self::Document, Self::Metadata>> {
        Some(indexes::get().sentence().native())
    }
}

impl SearchEngine for Engine {
    type Output = &'static Sentence;

    #[inline]
    fn doc_to_output(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<Self::Output>> {
        storage.sentences().by_id(inp.seq_id).map(|i| vec![i])
    }

    fn gen_query_vector(
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(Vector, String)> {
        let mut terms: HashSet<String> = HashSet::with_capacity(1);

        let indexer = index.get_indexer();

        if indexer.get_term(query).is_some() {
            terms.insert(query.to_string());
        } else {
            match sentence_reader::Parser::new(query).parse() {
                ParseResult::Sentence(s) => {
                    terms.extend(s.iter().map(|i| i.get_inflected()));
                    terms.extend(s.iter().map(|i| i.get_normalized()));
                }
                ParseResult::InflectedWord(w) => {
                    terms.insert(w.get_normalized());
                }
                ParseResult::None => (),
            };
        }

        terms.retain(|w| !index.is_stopword_cust(&w, 10.0).unwrap_or(true));

        let terms: Vec<_> = terms.into_iter().map(|i| format_query(&i)).collect();
        let vec = index.build_vector(&terms, Some(&TFIDF))?;
        Some((vec, query.to_string()))
    }
}

fn format_query(inp: &str) -> String {
    japanese::to_halfwidth(inp)
}
