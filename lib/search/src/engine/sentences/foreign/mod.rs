pub mod index;

use crate::engine::{
    document::SentenceDocument, metadata::Metadata, simple_gen_doc::GenDoc, Indexable, SearchEngine,
};
use resources::models::storage::ResourceStorage;
use types::jotoba::{languages::Language, sentences::Sentence};
use vector_space_model::DocumentVector;

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = Metadata;
    type Document = SentenceDocument;

    #[inline]
    fn get_index(
        language: Option<Language>,
    ) -> Option<&'static vector_space_model::Index<Self::Document, Self::Metadata>> {
        index::get(language.expect("Language not provided"))
    }
}

impl SearchEngine for Engine {
    type GenDoc = GenDoc;
    type Output = Sentence;

    #[inline]
    fn doc_to_output(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<&'static Self::Output>> {
        storage.sentences().by_id(inp.seq_id).map(|i| vec![i])
    }

    fn gen_query_vector(
        index: &vector_space_model::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(DocumentVector<Self::GenDoc>, String)> {
        let mut terms = all_terms(&query.to_lowercase());
        terms.push(query.to_string().to_lowercase());
        let query_document = GenDoc::new(terms);
        let doc = DocumentVector::new(index.get_indexer(), query_document.clone())?;
        Some((doc, query.to_string()))
    }
}

/// Splits a string into all its terms.
///
/// # Example
/// "make some coffee" => vec!["make","some","coffee"];
pub(crate) fn all_terms(i: &str) -> Vec<String> {
    i.split(' ')
        .map(|i| {
            format_word(i)
                .split(' ')
                .map(|i| i.to_lowercase())
                .filter(|i| !i.is_empty())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

/// Replaces all special characters into spaces so we can split it down into words
fn format_word(inp: &str) -> String {
    let mut out = String::from(inp);
    for i in ".,[]() \t\"'\\/-;:".chars() {
        out = out.replace(i, " ");
    }
    out
}
