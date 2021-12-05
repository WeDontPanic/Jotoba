use types::jotoba::{languages::Language, words::Word};

use crate::BaseData;

pub fn need_3dot(data: &BaseData, word: &Word) -> bool {
    word.get_inflections().is_some()
        || word.collocations.is_some()
        /*
        || word.get_intransitive_counterpart().is_some()
        || word.get_transitive_counterpart().is_some()
        */
        || (word.has_sentence(data.user_settings.user_lang)
            || (data.user_settings.show_english && word.has_sentence(Language::English)))
        || data.config.is_debug()
        || word.audio_file("ogg").is_some()
}
