use search::query::{parser::QueryParser, Tag, UserSettings};
use types::jotoba::search::SearchTarget;

pub(crate) fn parse(inp: String) -> Parsed {
    let query = QueryParser::new(inp.clone(), SearchTarget::Words, UserSettings::default()).parse();

    if query.is_none() {
        return Parsed::new(inp.to_string());
    }

    let query = query.unwrap();
    let tags = query.tags;
    Parsed::with_tags(query.query_str, tags)
}

pub(crate) struct Parsed {
    pub query: String,
    pub tags: Vec<Tag>,
}

impl Parsed {
    #[inline]
    fn new(query: String) -> Self {
        Self {
            query,
            tags: vec![],
        }
    }

    #[inline]
    fn with_tags(query: String, tags: Vec<Tag>) -> Self {
        Self { query, tags }
    }

    #[inline]
    pub fn search_target(&self) -> Option<SearchTarget> {
        self.tags
            .iter()
            .find(|i| i.is_search_type())
            .map(|i| i.as_search_type().unwrap())
            .copied()
    }
}
