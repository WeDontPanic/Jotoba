pub mod names;
pub mod radical;
pub mod result;
pub mod result_item;
pub mod search_task;
pub mod sentences;
pub mod utils;
pub mod words;

pub use search_task::SearchTask;

use ngindex::{build::weights::TermWeight, term_store::TermIndexer, VectorStore};
use search_task::sort_item::SortItem;
use std::hash::Hash;
use types::jotoba::languages::Language;
use vector_space_model2::{metadata::Metadata, traits::Decodable, Vector};

pub trait Index<D> {
    fn get_indexer(&self) -> &TermIndexer;
    fn get_vector_store(&self) -> &VectorStore<D>;
    fn build_vector(&self, terms: &[&str], something: Option<&dyn TermWeight>) -> Option<Vector>;
}

impl<T: Decodable, M> Index<T> for vector_space_model2::Index<T, M> {
    fn get_indexer(&self) -> &TermIndexer {
        self.get_indexer()
    }

    fn get_vector_store(&self) -> &VectorStore<T> {
        self.get_vector_store()
    }

    fn build_vector(&self, terms: &[&str], weight: Option<&dyn TermWeight>) -> Option<Vector> {
        self.build_vector(terms, weight)
    }
}

impl<T: Decodable> Index<T> for ngindex::NGIndex<T> {
    fn get_indexer(&self) -> &TermIndexer {
        self.index().get_indexer()
    }

    fn get_vector_store(&self) -> &VectorStore<T> {
        self.index().get_vector_store()
    }

    fn build_vector(&self, terms: &[&str], _: Option<&dyn TermWeight>) -> Option<Vector> {
        self.make_query_vec(terms[0])
    }
}

pub trait Indexable {
    type Metadata: Metadata + 'static;
    type Document: Decodable + Clone + 'static + Eq + Hash + Send;
    type Index: Index<Self::Document> + 'static;

    fn get_index(language: Option<Language>) -> Option<&'static Self::Index>;
}

pub trait DocumentGenerateable {
    fn new<T: ToString>(terms: Vec<T>) -> Self;
}

pub trait SearchEngine: Indexable {
    type Output: PartialEq + Eq + Hash + 'static + Send + Sync + Clone;

    /// Loads the corresponding Output type from a document
    fn doc_to_output(input: &Self::Document) -> Option<Vec<Self::Output>>;

    /// Generates a vector for a query, in order to be able to compare results with a vector
    fn gen_query_vector(
        index: &Self::Index,
        query: &str,
        align: bool,
        language: Option<Language>,
    ) -> Option<(Vector, String)>;

    fn align_query<'b>(
        _original: &'b str,
        _index: &Self::Index,
        _language: Option<Language>,
    ) -> Option<&'b str> {
        None
    }

    #[inline]
    fn score(item: SortItem<Self::Output>) -> usize {
        (item.vec_simiarity() * 100.0) as usize
    }

    fn query_formatted(inp: &str) -> String {
        inp.to_string()
    }
}
