use once_cell::sync::Lazy;
use regex::Regex;

pub const QUOTS_CONTENT: Lazy<Regex> = Lazy::new(|| Regex::new(r#""[^"]+""#).unwrap());

pub fn parse(inp: &str) -> (String, Vec<String>) {
    if !inp.contains('\"') {
        return (inp.to_string(), vec![]);
    }

    let mut terms = vec![];

    let mut new_query = inp.to_string();

    let mut delta = 0;
    for quots in QUOTS_CONTENT.find_iter(inp) {
        let r = quots.range();

        // strip quotes, we want the content
        let s = r.start - delta;
        let e = r.end - delta - 1;
        new_query.replace_range(s..s + 1, "");
        new_query.replace_range(e - 1..e, "");

        let s = r.start + 1;
        let e = r.end - 1;
        terms.push(inp[s..e].to_string().to_lowercase());
        delta += 2;
    }

    (new_query, terms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_quote_cnt() {
        let inp = r#"this is "some" text that "contains some" quotes" lol"#;
        let res = vec!["some", "contains some"];
        assert_eq!(parse(inp).1, res);
    }
}
