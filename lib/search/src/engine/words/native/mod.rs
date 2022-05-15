pub mod index;
pub mod regex;
pub mod regex_index;

use crate::engine::{Indexable, SearchEngine};
use resources::models::storage::ResourceStorage;
use types::jotoba::languages::Language;
use types::jotoba::words::Word;
use vector_space_model2::{DefaultMetadata, Vector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = u32;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model2::Index<Self::Document, Self::Metadata>> {
        Some(index::get())
    }
}

impl SearchEngine for Engine {
    type Output = &'static Word;

    #[inline]
    fn doc_to_output<'a>(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<Self::Output>> {
        storage.words().by_sequence(*inp).map(|i| vec![i])
    }

    fn gen_query_vector(
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(Vector, String)> {
        let mut terms = vec![(query.to_string(), 1.0)];

        let mut indexer = index.get_indexer().clone();
        let add_terms = tinysegmenter::tokenize(query)
            .into_iter()
            .filter_map(|term| {
                let indexed = indexer.find_term(&term)?;
                (indexed.doc_frequency() <= 5_000).then(|| (term, 0.03))
            })
            .collect::<Vec<_>>();

        terms.extend(add_terms);

        let vec = index.build_vector_weights(&terms)?;
        Some((vec, query.to_owned()))

        /*
        let query_document = GenDoc::new(vec![query]);
        let mut doc = DocumentVector::new(index.get_indexer(), query_document)?;

        // TODO: look if this makes the results really better. If not, remove
        let terms = tinysegmenter::tokenize(query);

        let mut indexer = index.get_indexer().clone();

        let terms = terms
            .into_iter()
            .filter_map(|term| {
                let indexed = indexer.find_term(&term)?;
                (indexed.get_frequency() <= 5_000).then(|| term)
            })
            .collect::<Vec<_>>();

        doc.add_terms(index.get_indexer(), &terms, true, Some(0.03));

        Some((doc, query.to_owned()))
        */
    }
}
