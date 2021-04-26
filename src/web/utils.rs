use crate::search::{query::Query, query_parser::QueryType};

/// Return a string 'selected' if the query_type in qs is equal to i
/// required by the base template
pub fn sel_str(qs: &Option<&Query>, i: QueryType) -> String {
    if qs
        .and_then(|query| (query.type_ == i).then(|| true))
        .is_some()
    {
        String::from("selected")
    } else {
        String::from("")
    }
}

/// Gets an owned String of the query
pub fn get_query_str(qs: &Option<&Query>) -> String {
    qs.map(|qs| qs.without_search_type_tags())
        .unwrap_or_default()
}
