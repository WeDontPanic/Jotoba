pub mod document;
pub mod metadata;
pub mod names;
pub mod result;
pub mod result_item;
pub mod search_task;
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

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        words::native::index::load(config1.get_indexes_source());
    }));

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        words::foreign::index::load(&config1).expect("failed to load index");
    }));

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        names::foreign::index::load(&config1);
    }));

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        names::native::index::load(&config1);
    }));

    for j in joins {
        j.join().map_err(|_| error::Error::Unexpected)?;
    }

    /*
    sentences::japanese::index::load(config);
    sentences::foreign::index::load(config)?;
    */

    Ok(())
}

pub trait Indexable {
    type Metadata: Metadata + 'static;
    type Document: Decodable + Clone + 'static;

    fn get_index(
        language: Option<Language>,
    ) -> Option<&'static Index<Self::Document, Self::Metadata>>;
}

pub trait SearchEngine: Indexable {
    type GenDoc: document_vector::Document;
    type Output: PartialEq + Eq + Hash;

    /// Loads the corresponding Output type from a document
    fn doc_to_output<'a>(
        storage: &'a ResourceStorage,
        input: &Self::Document,
    ) -> Option<Vec<&'a Self::Output>>;

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
