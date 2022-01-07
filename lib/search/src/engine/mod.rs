pub mod document;
pub mod guess;
pub mod kanji;
pub mod metadata;
pub mod names;
pub mod radical;
pub mod result;
pub mod result_item;
pub mod search_task;
pub mod sentences;
pub mod simple_gen_doc;
pub mod words;

use std::hash::Hash;

use config::Config;

use resources::models::storage::ResourceStorage;
pub use search_task::SearchTask;
use types::jotoba::languages::Language;
use vector_space_model::{
    document_vector, metadata::Metadata, traits::Decodable, DocumentVector, Index,
};

/// Load all indexes for word search engine
pub fn load_indexes(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let index_path = config.get_indexes_source().to_owned();

    rayon::scope(|s| {
        s.spawn(|_| {
            words::native::index::load(config.get_indexes_source());
        });
        s.spawn(|_| {
            words::native::regex_index::load(config.get_indexes_source());
        });
        s.spawn(|_| {
            words::foreign::index::load(index_path).expect("failed to load index");
        });
        s.spawn(|_| {
            names::foreign::index::load(&config);
        });
        s.spawn(|_| {
            names::native::index::load(&config);
        });
        s.spawn(|_| {
            sentences::native::index::load(&config);
        });
        s.spawn(|_| {
            radical::index::load(&config).expect("Failed to load radical index");
        });
        s.spawn(|_| {
            sentences::foreign::index::load(&config).expect("Failed to load index");
        });
    });

    Ok(())
}

pub trait Indexable {
    type Metadata: Metadata + 'static;
    type Document: Decodable + Clone + 'static + Eq + Hash + Send;

    fn get_index(
        language: Option<Language>,
    ) -> Option<&'static Index<Self::Document, Self::Metadata>>;
}

pub trait DocumentGenerateable {
    fn new<T: ToString>(terms: Vec<T>) -> Self;
}

pub trait SearchEngine: Indexable {
    type GenDoc: document_vector::Document + DocumentGenerateable + Send;
    type Output: PartialEq + Eq + Hash + 'static + Send + Sync;

    /// Loads the corresponding Output type from a document
    fn doc_to_output(
        storage: &'static ResourceStorage,
        input: &Self::Document,
    ) -> Option<Vec<&'static Self::Output>>;

    /// Generates a vector for a query, in order to be able to compare results with a vector
    fn gen_query_vector(
        index: &Index<Self::Document, Self::Metadata>,
        query: &str,
        align: bool,
        language: Option<Language>,
    ) -> Option<(DocumentVector<Self::GenDoc>, String)>;

    fn align_query<'b>(
        _original: &'b str,
        _index: &Index<Self::Document, Self::Metadata>,
        _language: Option<Language>,
    ) -> Option<&'b str> {
        None
    }
}
