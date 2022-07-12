use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[derive(
    Debug, PartialEq, Clone, Copy, AsRefStr, Serialize, Deserialize, EnumString, EnumIter, Hash, Eq,
)]
#[repr(u8)]
pub enum Tag {
    #[strum(serialize = "casual")]
    Casual,
    #[strum(serialize = "formal")]
    Formal,
    #[strum(serialize = "humble")]
    Humble,
    #[strum(serialize = "kansai", serialize = "kansai dialect")]
    Kansai,
    #[strum(serialize = "female", serialize = "female speaker")]
    Female,
    #[strum(serialize = "male", serialize = "male speaker")]
    Male,
    #[strum(serialize = "proverb")]
    Proverb,
    #[strum(serialize = "translatedproverb")]
    TranslatedProverb,
    #[strum(serialize = "quote")]
    Quote,
    #[strum(serialize = "pun", serialize = "japanese puns")]
    Pun,
    #[strum(serialize = "ok")]
    Ok,
    #[strum(serialize = "japanglish")]
    Japanglish,
    #[strum(serialize = "haiku")]
    Haiku,
    #[strum(serialize = "vulgar")]
    Vulgar,
    #[strum(serialize = "conversation")]
    Conversation,
    #[strum(serialize = "slang")]
    Slang,
    #[strum(serialize = "meme")]
    Meme,

    #[strum(serialize = "bungo")]
    /// 文語
    Bungo,

    #[strum(serialize = "dialectal")]
    Dialectal,
    #[strum(serialize = "poetry")]
    Poetry,
    #[strum(serialize = "game")]
    Game,
    #[strum(serialize = "manga")]
    Manga,
    #[strum(serialize = "lie")]
    Lie,
}

impl Tag {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Tag> {
        <Tag as IntoEnumIterator>::iter()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}
