pub mod document;
pub mod guess;
pub mod metadata;
pub mod names;
pub mod radical;
pub mod result;
pub mod result_item;
pub mod search_task;
pub mod sentences;
pub mod simple_gen_doc;
pub mod words;

use std::{hash::Hash, thread};

use config::Config;

use resources::{models::storage::ResourceStorage, parse::jmdict::languages::Language};
pub use search_task::SearchTask;
use vector_space_model::{
    document_vector, metadata::Metadata, traits::Decodable, DocumentVector, Index,
};

/// Load all indexes for word search engine
pub fn load_indexes(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut joins = Vec::with_capacity(5);

    let index_path = config.get_indexes_source().to_owned();

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        words::native::index::load(config1.get_indexes_source());
    }));

    joins.push(thread::spawn(move || {
        words::foreign::index::load(index_path).expect("failed to load index");
    }));

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        names::foreign::index::load(&config1);
    }));

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        names::native::index::load(&config1);
    }));

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        sentences::native::index::load(&config1);
    }));

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        radical::index::load(&config1).expect("Failed to load radical index");
    }));

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        sentences::foreign::index::load(&config1).expect("Failed to load index");
    }));

    for j in joins {
        j.join().map_err(|_| error::Error::Unexpected)?;
    }

    Ok(())
}

pub trait Indexable {
    type Metadata: Metadata + 'static;
    type Document: Decodable + Clone + 'static + Eq + Hash;

    fn get_index(
        language: Option<Language>,
    ) -> Option<&'static Index<Self::Document, Self::Metadata>>;
}

pub trait DocumentGenerateable {
    fn new<T: ToString>(terms: Vec<T>) -> Self;
}

pub trait SearchEngine: Indexable {
    type GenDoc: document_vector::Document + DocumentGenerateable;
    type Output: PartialEq + Eq + Hash + 'static;

    /// Loads the corresponding Output type from a document
    fn doc_to_output(
        storage: &'static ResourceStorage,
        input: &Self::Document,
    ) -> Option<Vec<&'static Self::Output>>;

    /// Generates a vector for a query, in order to be able to compare results with a vector
    fn gen_query_vector(
        index: &Index<Self::Document, Self::Metadata>,
        query: &str,
    ) -> Option<DocumentVector<Self::GenDoc>>;

    fn align_query<'b>(
        _original: &'b str,
        _index: &Index<Self::Document, Self::Metadata>,
        _language: Option<Language>,
    ) -> Option<&'b str> {
        None
    }
}
