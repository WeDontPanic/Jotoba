pub mod doc;
pub mod index;
pub mod output;

use crate::engine::{metadata::Metadata, Indexable, SearchEngine, SearchTask};
use resources::storage::ResourceStorage;
use types::jotoba::languages::Language;
use utils::to_option;
use vector_space_model2::{build::weights::TFIDF, Vector};

use self::{doc::FWordDoc, output::WordOutput};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = Metadata;
    type Document = FWordDoc;

    #[inline]
    fn get_index(
        language: Option<Language>,
    ) -> Option<&'static vector_space_model2::Index<Self::Document, Self::Metadata>> {
        index::get(language.expect("Language required"))
    }
}

impl SearchEngine for Engine {
    type Output = WordOutput;

    #[inline]
    fn doc_to_output(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<Self::Output>> {
        let out_items = inp
            .items
            .iter()
            .map(|i| {
                let word = storage.words().by_sequence(i.seq_id).unwrap();
                WordOutput::new(word, i.positions.clone())
            })
            .collect::<Vec<_>>();
        to_option(out_items)
    }

    fn gen_query_vector(
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        query: &str,
        allow_align: bool,
        language: Option<Language>,
    ) -> Option<(Vector, String)> {
        //let query_str = self.fixed_term(index).unwrap_or(self.get_query_str());
        let query_str = query.to_lowercase();

        // search query to document vector
        let mut terms = split_to_words(&query_str);

        // align query to index
        if allow_align {
            for term in terms.iter_mut() {
                if let Some(aligned) = Self::align_query(term, index, language) {
                    *term = aligned.to_string();
                    println!("Aligned: {} to {}", &query, term);
                }
            }
        }

        /*
        let doc_store = index.get_vector_store();
        let result_count = index
            .build_vector(&terms, None)
            .map(|i| {
                i.vec_indices()
                    .map(|dim| doc_store.dimension_size(dim))
                    .sum::<usize>()
            })
            .unwrap_or(0);

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

        Some((query, query_document.as_query()))
        */

        let vec = index.build_vector(&terms, Some(&TFIDF))?;
        Some((vec, query.to_string()))
    }

    fn align_query<'b>(
        original: &'b str,
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        language: Option<Language>,
    ) -> Option<&'b str> {
        let query_str = original;
        let mut indexer = index.get_indexer().clone();

        let has_term = indexer.find_term(&query_str).is_some()
            || indexer.find_term(&query_str.to_lowercase()).is_some();

        if has_term {
            return None;
        }

        let tree = index::get_term_tree(language?)?;
        let mut res = tree.find(&query_str.to_string(), 1);
        if res.is_empty() {
            res = tree.find(&query_str.to_string(), 2);
        }
        res.sort_by(|a, b| a.1.cmp(&b.1));
        res.get(0).map(|i| i.0.as_str())
    }
}

/// Guesses the language of `query`. Returns multiple if it can't be exactly determined cause of
/// same/similar words across multiple languages
pub fn guess_language(query: &str) -> Vec<Language> {
    let possible_langs = Language::iter_word()
        .filter(|language| {
            // Filter languages that can theoretically build valid document vectors
            Engine::gen_query_vector(index::get(*language).unwrap(), query, false, None).is_some()
        })
        .collect::<Vec<_>>();

    // Stopwords or short queries can have lots of possible languages, filter most unlikely
    // ones out
    if possible_langs.len() > 1 {
        let mut scored = Vec::with_capacity(possible_langs.len());

        for lang in &possible_langs {
            let mut guess_task = SearchTask::<Engine>::with_language(query, *lang);
            guess_task.set_align(false);
            let guess = guess_task.estimate_result_count().unwrap(); // Only fails if index is not loaded, which is never the case
            scored.push((*lang, guess.value));
        }

        let max = scored.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        // allow all languages which have >= than 40% of max estimated results
        let threshold = max.1 as f32 * 0.4f32;
        scored.retain(|(_, est)| (*est) as f32 >= threshold);
        return scored.into_iter().map(|i| i.0).collect::<Vec<_>>();
    }

    possible_langs
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, time::Instant};

    use config::{Config, SearchConfig, ServerConfig};

    use super::*;

    #[test]
    fn test_guess_lang() {
        load_data();
        let test_set = &[
            ("hausaufgabe", vec![Language::German]),
            ("Regen", vec![Language::German, Language::Dutch]),
            ("musique", vec![Language::French]),
            ("dog", vec![Language::English]),
            ("Möbel", vec![Language::German]),
            ("sugar", vec![Language::English]),
            ("to correct", vec![Language::English]),
        ];

        for (query, expected) in test_set {
            println!("testing query: {}", query);
            let start = Instant::now();
            assert_eq!(&guess_language(query), expected);
            println!("lang guessing: {:?}", start.elapsed());
        }
    }

    fn load_data() {
        // never do this in production!
        let mut config = Config::new(Some(PathBuf::from("../../data/config.toml"))).unwrap();
        config.search = Some(SearchConfig {
            indexes_source: Some(String::from("../../indexes")),
            search_timeout: None,
            suggestion_timeout: None,
            suggestion_sources: Some(String::from("../../suggestions")),
            report_queries_after: None,
        });

        config.server = ServerConfig {
            storage_data: Some(String::from("../../resources/storage_data")),
            ..ServerConfig::default()
        };

        /*
        resources::initialize_resources(
            config.get_storage_data_path().as_str(),
            config.get_radical_map_path().as_str(),
            config.get_sentences_path().as_str(),
            config.get_kreading_freq_path().as_str(),
        )
        .expect("Failed to load resources");
        */

        index::load("../../indexes").unwrap();
    }
}

/// Splits a gloss value into its words. Eg.: "make some coffee" => vec!["make","some coffee"];
pub(crate) fn split_to_words(i: &str) -> Vec<String> {
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
#[inline]
fn format_word(inp: &str) -> String {
    let mut out = String::from(inp);
    for i in ".,[]() \t\"'\\/-;:".chars() {
        out = out.replace(i, " ");
    }
    out
}
