pub mod names;
pub mod radical;
pub mod result;
pub mod result_item;
pub mod search_task;
pub mod sentences;
pub mod utils;
pub mod words;

pub use search_task::SearchTask;

use resources::storage::ResourceStorage;
use std::hash::Hash;
use types::jotoba::languages::Language;
use vector_space_model2::{metadata::Metadata, traits::Decodable, Index, Vector};

use self::search_task::sort_item::SortItem;

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
    type Output: PartialEq + Eq + Hash + 'static + Send + Sync + Clone;

    /// Loads the corresponding Output type from a document
    fn doc_to_output(
        storage: &'static ResourceStorage,
        input: &Self::Document,
    ) -> Option<Vec<Self::Output>>;

    /// Generates a vector for a query, in order to be able to compare results with a vector
    fn gen_query_vector(
        index: &Index<Self::Document, Self::Metadata>,
        query: &str,
        align: bool,
        language: Option<Language>,
    ) -> Option<(Vector, String)>;

    fn align_query<'b>(
        _original: &'b str,
        _index: &Index<Self::Document, Self::Metadata>,
        _language: Option<Language>,
    ) -> Option<&'b str> {
        None
    }

    #[inline]
    fn score(item: SortItem<Self::Output>) -> usize {
        (item.vec_simiarity() * 100.0) as usize
    }
}
