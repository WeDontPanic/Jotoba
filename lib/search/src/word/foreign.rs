use std::{collections::HashMap, io::Read};

use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use once_cell::sync::OnceCell;
use parse::jmdict::languages::Language;
use vector_space_model::{
    document_vector,
    traits::{Decodable, Encodable},
    Index,
};

static INDEX: OnceCell<HashMap<Language, Index<Document>>> = OnceCell::new();

pub fn load_index() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    let index = Index::<Document>::open("./out_index_de")?;
    map.insert(Language::German, index);

    INDEX.set(map).unwrap();

    Ok(())
}

pub(crate) async fn find(language: Language, query: &str) -> Vec<(usize, f32)> {
    let index = INDEX.get().unwrap().get(&language).unwrap();

    let term_indexer = index.get_indexer();
    let mut doc_store = index.get_vector_store();

    // search query to document vector
    let query_document = GenDoc::new(query, vec![]);
    let mut query = match document_vector::DocumentVector::new(term_indexer, query_document.clone())
    {
        Some(s) => s,
        None => {
            return vec![];
        }
    };

    let result_count = query
        .vector()
        .vec_indices()
        .map(|dim| doc_store.dimension_size(dim))
        .sum::<usize>();

    if result_count < 20 {
        // Add substrings of query to query document vector
        let sub_terms: Vec<_> = GenDoc::sub_documents(&query_document)
            .into_iter()
            .map(|i| document_vector::Document::get_terms(&i))
            .flatten()
            .collect();
        query.add_terms(term_indexer, &sub_terms, true, Some(0.3));
    }

    let document_vectors = doc_store
        .get_all_async(&query.vector().vec_indices().collect::<Vec<_>>())
        .await
        .unwrap();

    // Sort by relevance
    let mut found: Vec<_> = document_vectors
        .iter()
        .filter_map(|i| {
            let similarity = i.similarity(&query);
            (similarity != 0f32).then(|| (i, similarity))
        })
        .collect();

    // Sort by similarity to top
    found.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    found.dedup_by(|a, b| a.0.document == b.0.document);

    found
        .into_iter()
        .map(|i| i.0.document.seq_ids.iter().copied().map(move |j| (j, i.1)))
        .take(10)
        .flatten()
        .collect()
}

/// A retrieved document containing seq_ids and optionally the text of the document for debugging
/// purposes. This is retrieved from a `vector_store`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub seq_ids: Vec<usize>,
    pub text: String,
}

impl Decodable for Document {
    #[inline(always)]
    fn decode<T: ByteOrder, R: Read>(mut data: R) -> Result<Self, vector_space_model::Error> {
        let seq_id_count = data.read_u16::<T>()?;

        // Parse sequence IDs
        let seq_ids = (0..seq_id_count)
            .map(|_| data.read_u64::<T>().map(|i| i as usize))
            .collect::<Result<_, _>>()?;

        let text = String::new();

        Ok(Self { seq_ids, text })
    }
}

/// A `document_vector::Document` implementing type for generating new vectors
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct GenDoc {
    terms: Vec<String>,
    seq_ids: Vec<usize>,
}

impl Encodable for GenDoc {
    fn encode<T: ByteOrder>(&self) -> Result<Vec<u8>, vector_space_model::Error> {
        let mut encoded = vec![];

        encoded.write_u16::<T>(self.seq_ids.len() as u16)?;

        for seq_id in self.seq_ids.iter() {
            encoded.write_u64::<T>(*seq_id as u64)?;
        }

        Ok(encoded)
    }
}

impl GenDoc {
    /// Create a new GenDoc Document
    pub fn new<T: ToString>(term: T, seq_ids: Vec<usize>) -> Self {
        let terms = split_to_words(&term.to_string());
        GenDoc { terms, seq_ids }
    }

    /// Calculate sub_documents which represent substring or similar meanings
    pub(crate) fn sub_documents(document: &Self) -> impl Iterator<Item = GenDoc> + '_ {
        document
            .terms
            .iter()
            .map(|i| smaller_substrings(&i))
            .flatten()
            .map(|i| GenDoc::new(&i, vec![]))
    }
}

impl document_vector::Document for GenDoc {
    fn get_terms(&self) -> Vec<String> {
        self.terms.clone()
    }
}

const MIN_W_LEN: usize = 4;
fn smaller_substrings(inp: &str) -> Vec<String> {
    if inp.chars().count() <= MIN_W_LEN {
        return vec![inp.to_owned()];
    }

    let mut res = (MIN_W_LEN..=inp.len())
        .map(|i| inp.chars().take(i).collect::<String>())
        .collect::<Vec<_>>();

    if inp.contains('-') {
        res.push(inp.replace("-", ""));
        res.push(inp.replace("-", " "));
    }

    res
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
fn format_word(inp: &str) -> String {
    let mut out = String::from(inp);
    for i in ".,[]() \t\"'\\/-;:".chars() {
        out = out.replace(i, " ");
    }
    out
}
