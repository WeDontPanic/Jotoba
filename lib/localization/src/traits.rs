use std::fmt::Display;

use super::language::Language;
use super::TranslationDict;

impl Translatable for &'static str {
    #[inline]
    fn get_id(&self) -> &'static str {
        self
    }
}

impl TranslatablePlural for &'static str {
    #[inline]
    fn get_plural_id(&self) -> &'static str {
        self
    }
}

/// This trait allows any objects after implementation to be translated (in singular) using `dict`
pub trait Translatable {
    /// Has to return a unique MsgID which has to represent a msgid within the po file(s)
    fn get_id(&self) -> &'static str;

    /// Returns the singular translation of `msg_id` from the given catalog
    /// or `msg_id` itself if a translation does not exist.
    fn gettext<'a>(&self, dict: &'a TranslationDict, language: Option<Language>) -> &'a str {
        dict.gettext(self.get_id(), language)
    }

    /// Returns the singular translation of `msg_id` in the context `msg_context`
    /// or `msg_id` itself if a translation does not exist.
    fn pgettext<'a>(
        &self,
        dict: &'a TranslationDict,
        context: &'a str,
        language: Option<Language>,
    ) -> &'a str {
        dict.pgettext(context, self.get_id(), language)
    }

    /// Returns the singular translation of `msg_id` from the given catalog
    /// or `msg_id` itself if a translation does not exist.
    fn gettext_fmt<'a, T: Display + Sized + Clone>(
        &self,
        dict: &'a TranslationDict,
        values: &[T],
        language: Option<Language>,
    ) -> String {
        dict.gettext_fmt(self.get_id(), values, language)
    }

    /// Returns the singular translation of `msg_id` in the context `msg_context`
    /// or `msg_id` itself if a translation does not exist.
    fn pgettext_fmt<T: Display + Sized + Clone>(
        &self,
        dict: &TranslationDict,
        context: &str,
        values: &[T],
        language: Option<Language>,
    ) -> String {
        dict.pgettext_fmt(context, self.get_id(), values, language)
    }

    /// Like gettext but returns an owned string
    fn gettext_custom(&self, dict: &TranslationDict, language: Option<Language>) -> String {
        dict.gettext(self.get_id(), language).to_owned()
    }
}

/// This trait allows any objects after implementation to be translated (in plural) using `dict`
pub trait TranslatablePlural: Translatable {
    /// Has to return a unique MsgID which has to represent a msgid_plural within the po file(s)
    fn get_plural_id(&self) -> &'static str;

    /// Returns the singular translation of `msg_id` from the given catalog
    /// or `msg_id` itself if a translation does not exist.
    fn ngettext<'a>(
        &self,
        dict: &'a TranslationDict,
        n: u64,
        language: Option<Language>,
    ) -> &'a str {
        dict.ngettext(self.get_id(), self.get_plural_id(), n, language)
    }

    /// Returns the singular translation of `msg_id` in the context `msg_context`
    /// or `msg_id` itself if a translation does not exist.
    fn npgettext<'a>(
        &self,
        dict: &'a TranslationDict,
        context: &'a str,
        n: u64,
        language: Option<Language>,
    ) -> &'a str {
        dict.npgettext(context, self.get_id(), self.get_plural_id(), n, language)
    }

    /// Returns the singular translation of `msg_id` from the given catalog
    /// or `msg_id` itself if a translation does not exist.
    fn ngettext_fmt<'a, T: Display + Sized + Clone>(
        &self,
        dict: &'a TranslationDict,
        n: u64,
        values: &[T],
        language: Option<Language>,
    ) -> String {
        dict.ngettext_fmt(self.get_id(), self.get_plural_id(), n, values, language)
    }

    /// Returns the singular translation of `msg_id` in the context `msg_context`
    /// or `msg_id` itself if a translation does not exist.
    fn npgettext_fmt<T: Display + Sized + Clone>(
        &self,
        dict: &TranslationDict,
        context: &str,
        n: u64,
        values: &[T],
        language: Option<Language>,
    ) -> String {
        dict.npgettext_fmt(
            context,
            self.get_id(),
            self.get_plural_id(),
            n,
            values,
            language,
        )
    }
}
