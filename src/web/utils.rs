use super::search::{QueryStruct, QueryType};

/// Return a string 'selected' if the query_type in qs is equal to i
/// required by the base template
pub fn sel_str(qs: &Option<QueryStruct>, i: QueryType) -> String {
    if qs
        .as_ref()
        .and_then(|query_str| {
            query_str
                .search_type
                .and_then(|st| if st == i { Some(true) } else { None })
        })
        .is_some()
    {
        String::from("selected")
    } else {
        String::from("")
    }
}
