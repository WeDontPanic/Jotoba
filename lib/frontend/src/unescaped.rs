use std::{
    fmt::Display,
    io::{self, Write},
};

use crate::templates::ToHtml;

/// Unescaped owned String
pub type UnescapedString = Unescaped<String>;

/// Unescaped owned String
pub type UnescapedStr<'a> = Unescaped<&'a str>;

/// Write something unescaped
pub struct Unescaped<T: Display>(T);

impl ToHtml for Unescaped<String> {
    #[inline]
    fn to_html(&self, out: &mut dyn Write) -> io::Result<()> {
        write!(out, "{}", self.0)
    }
}

impl<'a> ToHtml for Unescaped<&'a str> {
    #[inline]
    fn to_html(&self, out: &mut dyn Write) -> io::Result<()> {
        write!(out, "{}", self.0)
    }
}

impl<'a> From<&'a str> for UnescapedStr<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Unescaped(s)
    }
}

impl From<String> for UnescapedString {
    #[inline]
    fn from(s: String) -> Self {
        Unescaped(s)
    }
}

impl Unescaped<String> {
    #[inline]
    pub fn new<T: ToString>(t: T) -> Self {
        Unescaped(t.to_string())
    }
}

impl<'a> Unescaped<&'a str> {
    #[inline]
    pub fn new(t: &'a str) -> Self {
        Unescaped(t)
    }
}

impl<T: Display> ToString for Unescaped<T> {
    #[inline]
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<T: Display> Into<String> for Unescaped<T> {
    #[inline]
    fn into(self) -> String {
        (&self).into()
    }
}

impl<T: Display> Into<String> for &Unescaped<T> {
    #[inline]
    fn into(self) -> String {
        format!("{}", self.0)
    }
}

impl<T: AsRef<str> + Display> AsRef<str> for Unescaped<T> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Unescaped<&'a str> {
    /// Returns a string reference of the unescaped value
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.0.as_ref()
    }
}

impl Unescaped<String> {
    /// Returns a string reference of the unescaped value
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}
