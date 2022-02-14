use crate::unescaped::UnescapedString;
use itertools::Itertools;

/// Set of tags which can be rendered as HTML
#[derive(Clone)]
pub struct TagSet {
    tags: Vec<Tag>,
}

#[derive(Clone, PartialEq)]
pub struct Tag {
    pub key: TagKey,
    pub value: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TagKey {
    Og(TagKeyName),
    Twitter(TagKeyName),
}

#[derive(Clone, Copy, PartialEq)]
pub enum TagKeyName {
    Title,
    Type,
    Description,
    URL,
    Card,
}

impl TagSet {
    /// Creates a new empty tag set
    #[inline]
    pub(crate) fn new() -> Self {
        TagSet { tags: vec![] }
    }

    /// Creates a new empty tag set with n capacity
    #[inline]
    pub(crate) fn with_capacity(cap: usize) -> Self {
        TagSet {
            tags: Vec::with_capacity(cap),
        }
    }

    /// Adds a new og tag to the `TagSet`
    #[inline]
    pub fn add_og<S: AsRef<str>>(&mut self, key: TagKeyName, value: S) {
        let key = TagKey::Og(key);
        self.add(Tag::new(key, value))
    }

    /// Adds a new twitter tag to the `TagSet`
    #[inline]
    pub fn add_twitter<S: AsRef<str>>(&mut self, key: TagKeyName, value: S) {
        let key = TagKey::Twitter(key);
        self.add(Tag::new(key, value))
    }

    /// Adds a tag to the `TagSet`
    #[inline]
    pub fn add(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    /// Sets the value of an og tag. Returns `None` if no og tag with `key` found
    #[inline]
    pub fn set_og_tag<S: AsRef<str>>(&mut self, key: TagKeyName, value: S) -> Option<()> {
        self.set_tag(TagKey::Og(key), value)
    }

    /// Sets the value of a twitter tag. Returns `None` if no twitter tag with `key` found
    #[inline]
    pub fn set_twitter_tag<S: AsRef<str>>(&mut self, key: TagKeyName, value: S) -> Option<()> {
        self.set_tag(TagKey::Twitter(key), value)
    }

    /// Sets the value of a tag. Returns `None` if no tag with `key` found
    #[inline]
    pub fn set_tag<S: AsRef<str>>(&mut self, key: TagKey, value: S) -> Option<()> {
        self.tags.iter_mut().find(|i| i.key == key)?.value = value.as_ref().to_string();
        Some(())
    }

    /// Render the `TagSet`
    #[inline]
    pub fn render(&self) -> String {
        self.tags.iter().map(|i| i.render()).join("\n\t")
    }

    /// Render the `TagSet` unescaped (for use in HTML)
    #[inline]
    pub fn render_unescaped(&self) -> UnescapedString {
        self.render().into()
    }
}

impl Tag {
    /// Creates a new tag
    pub fn new<S: AsRef<str>>(key: TagKey, value: S) -> Self {
        let value = value.as_ref().trim().to_string();
        Self { key, value }
    }

    /// Renders a single tag to HTML
    #[inline]
    pub fn render(&self) -> String {
        let key_attr = match self.key {
            TagKey::Og(og) => format!("property=\"og:{}\"", og.as_ref()),
            TagKey::Twitter(twitter) => format!("property=\"twitter:{}\"", twitter.as_ref()),
        };
        format!("<meta {key_attr} content=\"{}\"/>", self.value)
    }
}

impl TagKey {
    /// Create a new og tag key
    #[inline]
    pub fn new_og(tag_name: TagKeyName) -> Self {
        TagKey::Og(tag_name)
    }

    /// Create a new twitter key
    #[inline]
    pub fn new_twitter(tag_name: TagKeyName) -> Self {
        TagKey::Og(tag_name)
    }
}

impl AsRef<str> for TagKeyName {
    #[inline]
    fn as_ref(&self) -> &str {
        match self {
            TagKeyName::Title => "title",
            TagKeyName::Type => "type",
            TagKeyName::Description => "description",
            TagKeyName::URL => "url",
            TagKeyName::Card => "card",
        }
    }
}
