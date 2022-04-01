use std::time::Instant;

use autocomplete::SuggestionTask;
use japanese::guessing::{could_be_romaji, is_romaji_repl};
use resources::models::suggestions::foreign_words::ForeignSuggestion;
use utils::{binary_search::BinarySearchable, real_string_len};

use super::super::*;

/// Returns suggestions based on non japanese input
pub async fn suggestions(query: &Query, query_str: &str) -> Option<Vec<WordPair>> {
    let start = Instant::now();
    let engine = storage::FOREIGN_WORD_ENGINE.get()?;

    let mut task = SuggestionTask::new(30);
    let mut query = TaskQuery::new(query_str, 1.0);
    query.longest_prefix.allow = true;
    query.longest_prefix.limit = 10000;
    query.longest_prefix.max_steps = 10;
    query.longest_prefix.frequency_weight = 0.7;
    query.longest_prefix.relevance_multiplier = 0.0001;
    query.frequency_weight = 2.0;

    task.add_query(engine.new_query(query));

    if let Some(hira_query) = try_romaji(query_str) {
        let jp_engine = storage::JP_WORD_ENGINE.get().unwrap();
        let mut query = TaskQuery::new(hira_query, 0.1);
        query.frequency_weight = 13.0;
        let query = jp_engine.new_query(query);
        task.add_query(query);
    }

    let res = task.search();

    let out = res
        .into_iter()
        .map(|i| WordPair {
            primary: i.primary,
            secondary: i.secondary,
        })
        .collect::<Vec<_>>();
    println!("suggestions took: {:?}", start.elapsed());

    /*
    let lang = query.settings.user_lang;

    // Check if suggestions are available for the given language
    if !resources::get().suggestions().foreign_words(lang).is_some() {
        return None;
    }

    let query_str = query_str.trim().to_owned();
    */

    //Some(search(lang, &query_str))
    Some(out)
}

/// Returns Some(String) if `query_str` could be (part of) romaji search input and None if not
fn try_romaji(query_str: &str) -> Option<String> {
    let str_len = real_string_len(query_str);
    if str_len < 4 || query_str.contains(' ') {
        return None;
    }

    if let Some(v) = is_romaji_repl(query_str) {
        return Some(v.to_hiragana());
    }

    // 'n' is the only hiragana with with=1 in romaji so allow them
    // to be treated properly too
    let min_len = 4 - query_str.chars().filter(|i| *i == 'n').count();

    // Strip one to avoid switching between romaji/normal results
    if str_len > min_len {
        let prefix = strip_str_end(query_str, 1);
        if let Some(v) = is_romaji_repl(prefix) {
            return Some(v.to_hiragana());
        }
    }

    // shi ending needs more stripping but also more existing romaji to not
    // heavily overlap with other results
    if str_len >= min_len + 2
        && (query_str.ends_with("sh")
            || query_str.ends_with("ch")
            || query_str.ends_with("ts")
            || query_str.ends_with("hy")
            || query_str.ends_with("ky")
            || query_str.ends_with("ny")
            || query_str.ends_with("my"))
    {
        let prefix = strip_str_end(query_str, 2);
        if let Some(v) = is_romaji_repl(prefix) {
            return Some(v.to_hiragana());
        }
    }

    None
}

/// Returns a substring of `inp` with `len` amount of tailing characters being removed.
/// This works for non UTF-8 as well. If len > |inp| "" gets returned
pub fn strip_str_end(inp: &str, len: usize) -> &str {
    match inp.char_indices().rev().nth(len - 1).map(|i| i.0) {
        Some(end) => &inp[..end],
        None => "",
    }
}

fn search<'a>(main_lang: Language, query_str: &'a str) -> Vec<WordPair> {
    let mut res: Vec<_> = search_by_lang(main_lang, query_str, true)
        .map(|i| {
            let similarity =
                (strsim::jaro(&i.text.to_lowercase(), &query_str.to_lowercase()) * 100f64) as i32;
            (i, main_lang, similarity)
        })
        .take(50)
        .chain(
            search_by_lang(main_lang, &query_str.to_lowercase(), true).filter_map(|i| {
                let similarity = (strsim::jaro(&i.text.to_lowercase(), &query_str.to_lowercase())
                    * 90f64) as i32;
                Some((i, main_lang, similarity))
            }),
        )
        .chain(
            search_by_lang(main_lang, &utils::first_letter_upper(query_str), true).filter_map(
                |i| {
                    let similarity =
                        (strsim::jaro(&i.text.to_lowercase(), &query_str.to_lowercase()) * 90f64)
                            as i32;
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
                        ((strsim::jaro(&i.text, query_str) * 100f64) as i32).saturating_sub(10);
                    (i, Language::English, similarity)
                }),
        );
    }

    let query_len = real_string_len(query_str);
    if query_len >= 3 {
        let last_char = query_str.chars().last().unwrap();
        let is_romaji = could_be_romaji(query_str);
        let is_romaji_trim = &query_str[0..query_len - last_char.len_utf8()];

        if is_romaji || (could_be_romaji(is_romaji_trim) && is_romaji_trim.len() >= 4) {
            let mut query = query_str;
            if could_be_romaji(is_romaji_trim) && !is_romaji {
                query = is_romaji_trim;
            }

            let hira_query = utils::format_romaji_nn(query).to_hiragana();
            if let Some(hira_res) = super::native::suggest_words(&[&hira_query], &[]) {
                hira_res.into_iter().for_each(|i| {
                    let exact_match = i.0.primary == hira_query;

                    let score = if exact_match {
                        400i32
                    } else {
                        (((i.1 + 1) as f32 * 4f32).log2() + 65f32) as i32
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

            if res.len() < 10 {
                res.push((
                    ForeignSuggestion {
                        text: hira_query,
                        ..ForeignSuggestion::default()
                    },
                    Language::Japanese,
                    // show romaji on bottom
                    -1,
                ));
            }
        }
    }

    // Sort by text first since its needed for dedup
    res.sort_by(|a, b| a.0.text.cmp(&b.0.text));
    res.dedup_by(|a, b| a.0.text == b.0.text && a.0.secondary == b.0.secondary);

    res.sort_by(|a, b| a.2.cmp(&b.2).reverse());

    res.into_iter()
        .take(30)
        .map(|i| foreign_suggestion_to_pair(i.0))
        .collect()
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

#[inline]
fn foreign_suggestion_to_pair(suggestion: ForeignSuggestion) -> WordPair {
    WordPair {
        primary: suggestion.text,
        secondary: suggestion.secondary,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strip_end() {
        let inp = "これはかっこいいテキスト";
        assert_eq!(strip_str_end(inp, 1), "これはかっこいいテキス");
        assert_eq!(strip_str_end(inp, 2), "これはかっこいいテキ");
        assert_eq!(strip_str_end(inp, 3), "これはかっこいいテ");
    }
}
