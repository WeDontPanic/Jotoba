use resources::{
    models::{storage::ResourceStorage, words::Word},
    parse::jmdict::languages::Language,
};
use utils::to_option;
use vector_space_model::{document_vector, DocumentVector};

use crate::engine::{document::MultiDocument, metadata::Metadata, Indexable, SearchEngine};
use gen::GenDoc;

pub mod gen;
pub mod index;

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = Metadata;
    type Document = MultiDocument;

    #[inline]
    fn get_index(
        language: Option<Language>,
    ) -> Option<&'static vector_space_model::Index<Self::Document, Self::Metadata>> {
        index::get(language.expect("Language required"))
    }
}

impl SearchEngine for Engine {
    type GenDoc = GenDoc;
    type Output = Word;

    #[inline]
    fn doc_to_output(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<&'static Self::Output>> {
        to_option(
            inp.seq_ids
                .iter()
                .map(|i| storage.words().by_sequence(*i).unwrap())
                .collect(),
        )
    }

    fn gen_query_vector(
        index: &vector_space_model::Index<Self::Document, Self::Metadata>,
        query: &str,
    ) -> Option<DocumentVector<Self::GenDoc>> {
        //let query_str = self.fixed_term(index).unwrap_or(self.get_query_str());
        let query_str = query;

        let term_indexer = index.get_indexer();

        // search query to document vector
        let query_document = GenDoc::new(query_str, vec![]);
        let mut query = document_vector::DocumentVector::new(term_indexer, query_document.clone())?;

        let doc_store = index.get_vector_store();

        let result_count = query
            .vector()
            .vec_indices()
            .map(|dim| doc_store.dimension_size(dim))
            .sum::<usize>();

        if result_count < 15 {
            // Add substrings of query to query document vector
            let sub_terms: Vec<_> = GenDoc::sub_documents(&query_document)
                .into_iter()
                .map(|i| document_vector::Document::get_terms(&i))
                .flatten()
                .collect();

            query.add_terms(term_indexer, &sub_terms, true, Some(0.3));
        }

        Some(query)
    }

    fn align_query<'b>(
        original: &'b str,
        index: &vector_space_model::Index<Self::Document, Self::Metadata>,
        language: Option<Language>,
    ) -> Option<&'b str> {
        let query_str = original;
        let mut indexer = index.get_indexer().clone();

        let has_term = indexer.find_term(&query_str).is_some()
            || indexer.find_term(&query_str.to_lowercase()).is_some();

        if has_term {
            return None;
        }

        let tree = index::get_term_tree(language.unwrap())?;
        let mut res = tree.find(&query_str.to_string(), 1);
        if res.is_empty() {
            res = tree.find(&query_str.to_string(), 2);
        }
        res.sort_by(|a, b| a.1.cmp(&b.1));
        res.get(0).map(|i| i.0.as_str())
    }
}
