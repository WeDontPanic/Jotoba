use types::jotoba::language::Language;

/// Prefix of a search query. eg 'seq: 1234'
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SearchPrefix {
    /// A custom language prefix. Eg: 'rus: Россия'
    LangOverwrite(Language),
    /// Search by sequence-id within jmdict
    BySequence(u32),
}
