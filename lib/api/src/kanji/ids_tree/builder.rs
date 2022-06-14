use ids_parser::Origin;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use types::api::kanji::ids_tree::OutObject;

static STOP_RADICALS: Lazy<HashSet<char>> = Lazy::new(|| {
    japanese::radicals::RADICALS
        .iter()
        .map(|i| i.1)
        .flatten()
        .map(|i| i.chars().next().unwrap())
        .collect()
});

pub struct KanjiTreeBuilder;

impl KanjiTreeBuilder {
    pub fn build(&self, c: char) -> Option<OutObject> {
        let retrieve = resources::get().kanji();
        let ids_kanji = retrieve.ids(c)?;

        let mut out = OutObject::new(c.to_string());

        let radicals = ids_kanji.comp_by_lang(Origin::Japan)?.get_radicals();

        // exit condition
        if (radicals.len() == 1 && radicals[0] == c)
            || radicals.is_empty()
            || STOP_RADICALS.contains(&c)
        {
            return Some(out);
        }

        let mut visited_items = HashSet::new();

        for radical in radicals {
            if visited_items.contains(&radical) {
                continue;
            }
            if let Some(child) = self.build(radical) {
                out.add_child(child);
            }
            visited_items.insert(radical);
        }

        Some(out)
    }
}
