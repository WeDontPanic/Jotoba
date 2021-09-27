pub mod index;

use crate::engine::{document::SingleDocument, simple_gen_doc::GenDoc, Indexable, SearchEngine};
use resources::{
    models::{storage::ResourceStorage, words::Word},
    parse::jmdict::languages::Language,
};
use vector_space_model::{DefaultMetadata, DocumentVector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = SingleDocument;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model::Index<Self::Document, Self::Metadata>> {
        Some(index::get())
    }
}

impl SearchEngine for Engine {
    type GenDoc = GenDoc;
    type Output = Word;

    #[inline]
    fn doc_to_output<'a>(
        storage: &'a ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<&'a Self::Output>> {
        storage.words().by_sequence(inp.seq_id).map(|i| vec![i])
    }

    fn gen_query_vector(
        index: &vector_space_model::Index<Self::Document, Self::Metadata>,
        query: &str,
    ) -> Option<DocumentVector<Self::GenDoc>> {
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

        Some(doc)
    }
}
