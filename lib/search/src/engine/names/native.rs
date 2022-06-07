use crate::engine::{Indexable, SearchEngine};
use resources::storage::ResourceStorage;
use types::jotoba::{languages::Language, names::Name};
use utils::to_option;
use vector_space_model2::{DefaultMetadata, Vector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = Vec<u32>;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model2::Index<Self::Document, Self::Metadata>> {
        Some(indexes::get().name().native())
    }
}

impl SearchEngine for Engine {
    type Output = &'static Name;

    #[inline]
    fn doc_to_output(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<Self::Output>> {
        to_option(
            inp.iter()
                .map(|i| storage.names().by_sequence(*i).unwrap())
                .collect(),
        )
    }

    fn gen_query_vector(
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(Vector, String)> {
        let vec = index.build_vector(&[query], None)?;
        Some((vec, query.to_owned()))
    }
}
