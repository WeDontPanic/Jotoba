pub mod error;
pub mod language;
pub mod traits;

use std::{collections::HashMap, fmt::Display, fs::File, str::FromStr};

use error::Error;
use gettext::Catalog;

use language::Language;
use log::{debug, error};

/// A Dictionary of multiple catalogs assigned to its languages. Requires at least one cataloge
/// for the defined [`default_lang`]
pub struct TranslationDict {
    catalogs: HashMap<Language, Catalog>,
    default_lang: Language,
}

impl TranslationDict {
    /// Creates a new [`TranslationDict`] value with the catalogs available in [`path`]. Parses the
    /// file names based into their representing [`Language`].
    pub fn new(path: &str, default_lang: Language) -> Result<TranslationDict, Error> {
        let mut catalogs = HashMap::new();

        debug!("Loading locales from: {}", path);

        // Initialize catalogs
        for file in std::fs::read_dir(path)? {
            let file = file?;
            let file_path = file.path();
            let stem = file_path.file_stem().unwrap().to_str().unwrap();

            // Ignore non .mo files
            if file_path
                .extension()
                .and_then(|ext| ext.to_str())
                .and_then(|ext| (ext.ends_with("mo")).then(|| 1))
                .is_none()
            {
                continue;
            }

            if let Ok(language) = Language::from_str(stem) {
                let catalog = Catalog::parse(File::open(file_path)?)?;
                catalogs.insert(language, catalog);

                debug!("Loaded locale: {:?}", language);
            } else {
                error!("Unknown language: {}", stem);
            }
        }

        // Check if `default_lang` is included
        if catalogs.get(&default_lang).is_none() {
            return Err(Error::DefaultNotFound);
        }

        Ok(TranslationDict {
            catalogs,
            default_lang,
        })
    }

    /// Returns the singular translation of `msg_id` from the given catalog
    /// or `msg_id` itself if a translation does not exist.
    pub fn gettext<'a>(&'a self, msg_id: &'a str, language: Option<Language>) -> &'a str {
        self.get_catalog(language).gettext(msg_id)
    }

    /// Returns the plural translation of `msg_id` from the given catalog
    /// with the correct plural form for the number `n` of objects.
    /// Returns msg_id if a translation does not exist and `n == 1`,
    /// msg_id_plural otherwise.
    pub fn ngettext<'a>(
        &'a self,
        msg_id: &'a str,
        msg_id_plural: &'a str,
        n: u64,
        language: Option<Language>,
    ) -> &'a str {
        self.get_catalog(language)
            .ngettext(msg_id, msg_id_plural, n)
    }

    /// Returns the singular translation of `msg_id`
    /// in the context `msg_context`
    /// or `msg_id` itself if a translation does not exist.
    pub fn pgettext<'a>(
        &'a self,
        msg_context: &'a str,
        msg_id: &'a str,
        language: Option<Language>,
    ) -> &'a str {
        self.get_catalog(language).pgettext(msg_context, msg_id)
    }

    /// Returns the plural translation of `msg_id` in the context `msg_context`
    /// with the correct plural form for the number `n` of objects.
    /// Returns msg_id if a translation does not exist and `n == 1`,
    /// msg_id_plural otherwise.
    pub fn npgettext<'a>(
        &'a self,
        msg_context: &'a str,
        msg_id: &'a str,
        msg_id_plural: &'a str,
        n: u64,
        language: Option<Language>,
    ) -> &'a str {
        self.get_catalog(language)
            .npgettext(msg_context, msg_id, msg_id_plural, n)
    }

    /// Returns the singular translation of `msg_id` from the given catalog
    /// or `msg_id` itself if a translation does not exist.
    pub fn gettext_fmt<T: Display + Sized + Clone>(
        &self,
        msg_id: &str,
        values: &[T],
        language: Option<Language>,
    ) -> String {
        format(self.gettext(msg_id, language), values)
    }

    /// Returns the plural translation of `msg_id` from the given catalog
    /// with the correct plural form for the number `n` of objects.
    /// Returns msg_id if a translation does not exist and `n == 1`,
    /// msg_id_plural otherwise.
    pub fn ngettext_fmt<T: Display + Sized + Clone>(
        &self,
        msg_id: &str,
        msg_id_plural: &str,
        n: u64,
        values: &[T],
        language: Option<Language>,
    ) -> String {
        format(self.ngettext(msg_id, msg_id_plural, n, language), values)
    }

    /// Returns the singular translation of `msg_id`
    /// in the context `msg_context`
    /// or `msg_id` itself if a translation does not exist.
    pub fn pgettext_fmt<T: Display + Sized + Clone>(
        &self,
        msg_context: &str,
        msg_id: &str,
        values: &[T],
        language: Option<Language>,
    ) -> String {
        format(self.pgettext(msg_context, msg_id, language), values)
    }

    /// Returns the plural translation of `msg_id` in the context `msg_context`
    /// with the correct plural form for the number `n` of objects.
    /// Returns msg_id if a translation does not exist and `n == 1`,
    /// msg_id_plural otherwise.
    pub fn npgettext_fmt<T: Display + Sized + Clone>(
        &self,
        msg_context: &str,
        msg_id: &str,
        msg_id_plural: &str,
        n: u64,
        values: &[T],
        language: Option<Language>,
    ) -> String {
        format(
            self.npgettext(msg_context, msg_id, msg_id_plural, n, language),
            values,
        )
    }

    /// Returns the catalog for the given language
    pub fn get_catalog(&self, language: Option<Language>) -> &Catalog {
        let language = language.unwrap_or_default();
        self.catalogs
            .get(&language)
            .unwrap_or_else(|| self.get_default_catalog())
    }

    /// Returns the default catalog
    pub fn get_default_catalog(&self) -> &Catalog {
        self.catalogs
            .get(&self.default_lang)
            .expect("Missing default catalog")
    }
}

/// Formats the input with the passed values and returns a newly allocated owned String
fn format<T: Display + Sized + Clone>(inp: &str, values: &[T]) -> String {
    use dyn_fmt::AsStrFormatExt;

    let placeholder_count = count_placeholder(inp);
    if placeholder_count != values.len() {
        if values.len() == 1 {
            let first = values[0].clone();
            let mut values = values.to_vec();
            for _ in 0..placeholder_count - 1 {
                values.push(first.clone());
            }
            return inp.format(&values);
        }
    }

    inp.format(values)
}

fn count_placeholder(inp: &str) -> usize {
    inp.matches("{}").count()
}
