use byteorder::{ByteOrder, WriteBytesExt};
use vector_space_model::{document_vector, traits::Encodable};

use crate::engine::DocumentGenerateable;

const MIN_W_LEN: usize = 4;

/// A `document_vector::Document` implementing type for generating new vectors
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct GenDoc {
    terms: Vec<String>,
    seq_ids: Vec<usize>,
}

impl GenDoc {
    /// Create a new GenDoc Document
    #[inline]
    pub fn new<T: ToString>(term: T, seq_ids: Vec<usize>) -> Self {
        let terms = split_to_words(&term.to_string());
        GenDoc { terms, seq_ids }
    }

    /// Calculate sub_documents which represent substring or similar meanings
    #[inline]
    pub(crate) fn sub_documents(document: &Self) -> impl Iterator<Item = GenDoc> + '_ {
        document
            .terms
            .iter()
            .map(|i| smaller_substrings(&i))
            .flatten()
            .map(|i| GenDoc::new(&i, vec![]))
    }

    #[inline]
    pub fn get_terms(&self) -> &Vec<String> {
        &self.terms
    }

    pub fn get_terms_mut(&mut self) -> &mut Vec<String> {
        &mut self.terms
    }

    pub fn as_query(&self) -> String {
        self.terms.join(" ")
    }
}

impl DocumentGenerateable for GenDoc {
    fn new<T: ToString>(terms: Vec<T>) -> Self {
        let terms = terms.into_iter().map(|i| i.to_string()).collect::<Vec<_>>();
        GenDoc {
            terms,
            seq_ids: vec![],
        }
    }
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

impl document_vector::Document for GenDoc {
    #[inline]
    fn get_terms(&self) -> Vec<String> {
        self.terms.clone()
    }
}

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
#[inline]
fn format_word(inp: &str) -> String {
    let mut out = String::from(inp);
    for i in ".,[]() \t\"'\\/-;:".chars() {
        out = out.replace(i, " ");
    }
    out
}
