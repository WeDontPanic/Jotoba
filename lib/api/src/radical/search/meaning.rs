pub fn search(query: &str) -> Vec<char> {
    if query.len() <= 2 {
        return vec![];
    }

    let kanji_retr = resources::get().kanji();

    // TODO: implement better search algo for names (maybe map them by hand and add aliases for
    // better search experience)
    kanji_retr
        .radicals()
        .filter_map(|rad| {
            let translations = rad.translations.as_ref()?;
            translations.contains(&query.to_lowercase()).then(|| rad)
        })
        .map(|i| i.literal)
        .collect()
}
