mod document;
mod gen;
mod metadata;

use self::document::Document;
use crate::query::Query;
use config::Config;
use error::Error;
use gen::GenDoc;
use log::{debug, error};
use metadata::Metadata;
use once_cell::sync::OnceCell;
use parse::jmdict::languages::Language;
use std::{cmp::Ordering, collections::HashMap};
use vector_space_model::{document_vector, DocumentVector};

type Index = vector_space_model::Index<Document, Metadata>;

static INDEXES: OnceCell<HashMap<Language, Index>> = OnceCell::new();

/// A single result item for `find`
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct FindResult {
    pub(super) seq_id: usize,
    pub(super) relevance: f32,
    pub(super) language: Language,
}

impl Eq for FindResult {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for FindResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.relevance
            .partial_cmp(&other.relevance)
            .map(|i| i.reverse())
    }
}

impl Ord for FindResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap_or(Ordering::Equal)
    }
}

/// Load all available indexes
pub fn load_indexes(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // All index files in index source folder
    let index_files = std::fs::read_dir(config.get_indexes_source()).and_then(|i| {
        i.map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
    })?;

    let mut map = HashMap::new();

    for index_file in index_files {
        let index = match Index::open(&index_file) {
            Ok(index) => index,
            Err(err) => {
                let file = index_file.display();
                error!("Failed to load index \"{}\": {:?}", file, err);
                continue;
            }
        };

        let lang = index.get_metadata().language;
        map.insert(lang, index);
        debug!("Loaded index file: {:?}", lang);
    }

    if map.is_empty() {
        panic!("No index file loaded");
    }

    INDEXES.set(map).unwrap();

    Ok(())
}

pub(super) struct Find<'a> {
    limit: usize,
    offset: usize,
    query: &'a Query,
}

impl<'a> Find<'a> {
    #[inline]
    pub(super) fn new(query: &'a Query, limit: usize, offset: usize) -> Self {
        Self {
            limit,
            offset,
            query,
        }
    }

    /// Do a foreign word search
    pub(super) async fn find(&self) -> Result<Vec<FindResult>, Error> {
        let language = self.query.settings.user_lang;

        let native_results = self.find_lang(language).await?;
        if self.get_lang() == Language::default() {
            return Ok(native_results);
        }

        let english_results = self.find_lang(Language::default()).await?;

        let mut result = native_results
            .into_iter()
            .chain(english_results.into_iter())
            .collect::<Vec<_>>();

        result.sort();
        result.dedup();

        Ok(result)
    }

    pub(super) async fn find_lang(&self, language: Language) -> Result<Vec<FindResult>, Error> {
        let index = match INDEXES.get().and_then(|indexes| indexes.get(&language)) {
            Some(index) => index,
            None => return Ok(vec![]),
        };

        // VecStore is surrounded by an Arc
        let mut doc_store = index.get_vector_store().clone();

        let query_vec = match Self::gen_query_vec(self.get_query_str(), &index) {
            Some(vec) => vec,
            None => return Ok(vec![]),
        };

        // All vectors in queries dimensions
        let dimensions = query_vec.vector().vec_indices().collect::<Vec<_>>();
        let document_vectors = doc_store
            .get_all_async(&dimensions)
            .await
            .map_err(|_| error::Error::NotFound)?;

        // Sort by relevance
        let mut found: Vec<_> = document_vectors
            .iter()
            .filter_map(|i| {
                let similarity = i.similarity(&query_vec);
                (similarity != 0f32).then(|| (i, similarity))
            })
            .collect();

        // Sort by similarity to top
        found.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal).reverse());
        found.dedup_by(|a, b| a.0.document == b.0.document);

        let res = found
            .into_iter()
            .map(|i| i.0.document.seq_ids.iter().copied().map(move |j| (j, i.1)))
            .skip(self.offset)
            .take(self.limit)
            .flatten()
            .map(|i| FindResult {
                seq_id: i.0,
                relevance: i.1,
                language,
            })
            .collect::<Vec<_>>();

        Ok(res)
    }

    #[inline]
    fn get_query_str(&self) -> &str {
        &self.query.query
    }

    #[inline]
    fn get_lang(&self) -> Language {
        self.query.settings.user_lang
    }

    /// Generates a `DocumentVector<GenDoc>` for the given query
    fn gen_query_vec(query_str: &str, index: &Index) -> Option<DocumentVector<GenDoc>> {
        let term_indexer = index.get_indexer();
        let doc_store = index.get_vector_store();

        // search query to document vector
        let query_document = GenDoc::new(query_str, vec![]);
        let mut query = document_vector::DocumentVector::new(term_indexer, query_document.clone())?;

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
}
