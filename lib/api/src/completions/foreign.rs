use resources::models::suggestions::foreign_words::ForeignSuggestion;
use utils::binary_search::BinarySearchable;

use super::*;

/// Returns suggestions based on non japanese input
pub(super) async fn suggestions(query: &Query, query_str: &str) -> Option<Vec<WordPair>> {
    let lang = query.settings.user_lang;

    // Check if suggestions are available for the given language
    if !resources::get().suggestions().foreign_words(lang).is_some() {
        return None;
    }

    let query_str = query_str.to_owned();
    let res = actix_web::web::block(move || search(lang, &query_str))
        .await
        .unwrap();

    Some(res)
}

fn search<'a>(main_lang: Language, query_str: &'a str) -> Vec<WordPair> {
    let mut res: Vec<_> = search_by_lang(main_lang, query_str, true)
        .map(|i| {
            let similarity = (strsim::jaro(&i.text, query_str) * 100f64) as u32;
            (i, main_lang, similarity)
        })
        .take(50)
        .chain(
            search_by_lang(main_lang, &query_str.to_lowercase(), true).map(|i| {
                let similarity = (strsim::jaro(&i.text, &query_str.to_lowercase()) * 100f64) as u32;
                (i, main_lang, similarity)
            }),
        )
        .chain(
            search_by_lang(main_lang, &utils::first_letter_upper(query_str), true).map(|i| {
                let similarity =
                    (strsim::jaro(&i.text, &utils::first_letter_upper(query_str)) * 100f64) as u32;
                (i, main_lang, similarity)
            }),
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

    // Sort by text first since its needed for dedup
    res.sort_by(|a, b| a.0.text.cmp(&b.0.text));
    res.dedup_by(|a, b| a.0.text == b.0.text);

    res.sort_by(|a, b| a.2.cmp(&b.2).reverse());

    res.into_iter().take(10).map(|i| i.0.into()).collect()
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

        let found = loop {
            if substr.len() < 2 {
                break false;
            }
            if dict
                .binary_search_by(move |e: &ForeignSuggestion| beg_match(e, substr))
                .is_some()
            {
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
                dict.search(move |e: &ForeignSuggestion| beg_match(e, substr))
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
            primary: suggestion.text.to_owned(),
            secondary: None,
        }
    }
}
