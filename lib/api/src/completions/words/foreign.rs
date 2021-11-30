use japanese::guessing::could_be_romaji;
use resources::models::suggestions::foreign_words::ForeignSuggestion;
use utils::{binary_search::BinarySearchable, real_string_len};

use super::super::*;

/// Returns suggestions based on non japanese input
pub async fn suggestions(query: &Query, query_str: &str) -> Option<Vec<WordPair>> {
    let lang = query.settings.user_lang;

    // Check if suggestions are available for the given language
    if !resources::get().suggestions().foreign_words(lang).is_some() {
        return None;
    }

    let query_str = query_str.trim().to_owned();

    Some(search(lang, &query_str))
}

fn search<'a>(main_lang: Language, query_str: &'a str) -> Vec<WordPair> {
    let mut res: Vec<_> = search_by_lang(main_lang, query_str, true)
        .map(|i| {
            let similarity =
                (strsim::jaro(&i.text.to_lowercase(), &query_str.to_lowercase()) * 100f64) as u32;
            (i, main_lang, similarity)
        })
        .take(50)
        .chain(
            search_by_lang(main_lang, &query_str.to_lowercase(), true).filter_map(|i| {
                let similarity = (strsim::jaro(&i.text.to_lowercase(), &query_str.to_lowercase())
                    * 90f64) as u32;
                Some((i, main_lang, similarity))
            }),
        )
        .chain(
            search_by_lang(main_lang, &utils::first_letter_upper(query_str), true).filter_map(
                |i| {
                    let similarity =
                        (strsim::jaro(&i.text.to_lowercase(), &query_str.to_lowercase()) * 90f64)
                            as u32;
                    Some((i, main_lang, similarity))
                },
            ),
        )
        .collect();

    if main_lang != Language::English {
        res.extend(
            search_by_lang(Language::English, query_str, false)
                .take(100)
                .map(|i| {
                    let similarity =
                        ((strsim::jaro(&i.text, query_str) * 100f64) as u32).saturating_sub(10);
                    (i, Language::English, similarity)
                }),
        );
    }

    let query_len = real_string_len(query_str);
    if query_len >= 3 {
        let last_char = query_str.chars().last().unwrap();
        let is_romaji = could_be_romaji(query_str);
        let is_romaji_trim = &query_str[0..query_len - last_char.len_utf8()];

        if is_romaji || could_be_romaji(is_romaji_trim) {
            let mut query = query_str;
            if could_be_romaji(is_romaji_trim) && !is_romaji {
                query = is_romaji_trim;
            }

            let hira_query = query.to_hiragana();
            if let Some(hira_res) = super::native::suggest_words(&[&hira_query]) {
                hira_res.into_iter().for_each(|i| {
                    let exact_match = i.0.primary == hira_query;

                    let score = if exact_match {
                        400u32
                    } else {
                        (((i.1 + 1) as f32 * 4f32).log2() + 65f32) as u32
                    };

                    res.push((
                        ForeignSuggestion {
                            secondary: i.0.secondary,
                            text: i.0.primary,
                            occurrences: i.1,
                            ..ForeignSuggestion::default()
                        },
                        Language::Japanese,
                        score,
                    ));
                });
            }

            if res.is_empty() {
                res.push((
                    ForeignSuggestion {
                        text: query.to_hiragana(),
                        ..ForeignSuggestion::default()
                    },
                    Language::Japanese,
                    0,
                ));
            }
        }
    }

    // Sort by text first since its needed for dedup
    res.sort_by(|a, b| a.0.text.cmp(&b.0.text));
    res.dedup_by(|a, b| a.0.text == b.0.text && a.0.secondary == b.0.secondary);

    res.sort_by(|a, b| a.2.cmp(&b.2).reverse());

    res.into_iter().take(30).map(|i| i.0.into()).collect()
}

fn search_by_lang<'a>(
    lang: Language,
    query_str: &'a str,
    tryhard: bool,
) -> impl Iterator<Item = ForeignSuggestion> + 'a {
    let suggestion_provider = resources::get().suggestions();
    let dict = suggestion_provider.foreign_words(lang).unwrap();

    let find = move |e: &ForeignSuggestion| beg_match(e, query_str);

    let mut res = dict.search(find).take(10).collect::<Vec<_>>();

    if res.len() < 10 && query_str.len() > 1 && tryhard {
        let mut substr = query_str;

        let mut last_search = substr;

        let found = loop {
            if substr.len() < 4 {
                break false;
            }
            let end = substr.len() - (substr.len() % 2);
            if dict
                .binary_search_by(move |e: &ForeignSuggestion| beg_match(e, &substr[0..end]))
                .is_some()
            {
                last_search = &substr[0..end];
                break true;
            }

            let end = substr.char_indices().rev().nth(1);
            if end.is_none() {
                break false;
            }
            let end = end.unwrap();
            substr = &substr[0..end.0];
        };

        if found {
            res.extend(
                dict.search(move |e: &ForeignSuggestion| beg_match(e, last_search))
                    .take(100),
            );
        }
    }

    res.into_iter()
}

fn beg_match(e: &ForeignSuggestion, inp: &str) -> Ordering {
    if e.text.starts_with(&inp) {
        Ordering::Equal
    } else {
        e.text.as_str().cmp(&inp)
    }
}

impl From<ForeignSuggestion> for WordPair {
    #[inline]
    fn from(suggestion: ForeignSuggestion) -> Self {
        WordPair {
            primary: suggestion.text,
            secondary: suggestion.secondary,
        }
    }
}
