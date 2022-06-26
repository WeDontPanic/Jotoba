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

pub struct KanjiTreeBuilder {
    build_full: bool,
}

impl KanjiTreeBuilder {
    /// Creates a new TreeBuilder. The parameter specifies whether a full tree should be bulit or
    /// Only one which is restricted to the Radicals used in the radical picker
    pub fn new(build_full: bool) -> Self {
        Self { build_full }
    }

    /// Recursive method to build the OutObjects
    pub fn build(&self, c: char) -> Option<OutObject> {
        let retrieve = resources::get().kanji();
        let ids_kanji = retrieve.ids(c)?;

        let mut out = OutObject::new(c);

        out.set_literal_available(retrieve.has_literal(c));

        //let radicals = ids_kanji.comp_by_lang(Origin::Japan)?.get_radicals();
        let comps = match ids_kanji.comp_by_lang(Origin::Japan) {
            Some(s) => s,
            None => {
                if ids_kanji.compositions.len() == 1 {
                    &ids_kanji.compositions[0]
                } else {
                    return None;
                }
            }
        };
        let radicals = comps.get_radicals();

        // recursive exit condition
        if (radicals.len() == 1 && radicals[0] == c)
            || radicals.is_empty()
            || (STOP_RADICALS.contains(&c) && !self.build_full)
        {
            return Some(out);
        }

        let mut visited_items = HashSet::with_capacity(radicals.len());

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
