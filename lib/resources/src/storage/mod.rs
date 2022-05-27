pub mod feature;
pub mod kanji;
pub mod name;
pub mod sentence;
pub mod word;

use super::retrieve::{
    kanji::KanjiRetrieve, name::NameRetrieve, sentence::SentenceRetrieve, word::WordRetrieve,
};

use self::{
    feature::Feature, kanji::KanjiStorage, name::NameStorage, sentence::SentenceStorage,
    word::WordStorage,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

/// Storage holding all data of Jotoba
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ResourceStorage {
    pub words: WordStorage,
    pub kanji: KanjiStorage,
    pub names: NameStorage,
    pub sentences: SentenceStorage,
}

impl ResourceStorage {
    /// Create a new empty `ResourceStorage`
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if all necessary features are present
    pub fn check(&self) -> bool {
        self.missing_but_required().is_empty()
    }

    pub fn missing_but_required(&self) -> Vec<Feature> {
        let missing = self.missing_features();
        let mut out = vec![];

        for req_feature in super::REQUIRED_FEATURES {
            if missing.contains(req_feature) {
                out.push(*req_feature);
            }
        }

        out
    }

    /// Returns a list of features that are missing but required
    pub fn missing_features(&self) -> Vec<Feature> {
        let features = self.get_features();

        let mut missing = vec![];

        for feature in Feature::iter() {
            if !features.contains(&feature) {
                missing.push(feature);
            }
        }

        missing
    }

    /// Returns `true` if ResourceStorage has the given feature
    #[inline]
    pub fn has_feature(&self, feature: Feature) -> bool {
        self.get_features().contains(&feature)
    }

    /// Returns a list of all features of the ResourceStorage's data
    pub fn get_features(&self) -> Vec<Feature> {
        let mut out = vec![];
        out.extend(self.words.get_features());
        out.extend(self.kanji.get_features());
        out.extend(self.names.get_features());
        out.extend(self.sentences.get_features());
        out
    }
}

// Retrieve functions
// `ResourceStorage::check` is supposed to be called at the begininng to ensure
// those fields are not unset
impl ResourceStorage {
    /// Get a reference to the resource storage's words.
    #[inline(always)]
    pub fn words<'a>(&'a self) -> WordRetrieve<'a> {
        WordRetrieve::new(&self.words)
    }

    /// Get a reference to the resource storage's kanji.
    #[inline(always)]
    pub fn kanji(&self) -> KanjiRetrieve {
        KanjiRetrieve::new(&self.kanji)
    }

    /// Get a reference to the resource storage's names.
    #[inline(always)]
    pub fn names(&self) -> NameRetrieve {
        NameRetrieve::new(&self.names)
    }

    /// Get a reference to the resource storage's sentences.
    #[inline(always)]
    pub fn sentences(&self) -> SentenceRetrieve {
        SentenceRetrieve::new(&self.sentences)
    }
}
