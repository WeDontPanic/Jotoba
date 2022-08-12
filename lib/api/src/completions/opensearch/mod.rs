mod parse;

use actix_web::web;
use serde::Deserialize;
use types::{api::completions::Request, jotoba::search::SearchTarget};

#[derive(Deserialize)]
pub struct EPQuery {
    q: String,
}

pub async fn suggestion_ep(query: web::Query<EPQuery>) -> Result<String, actix_web::Error> {
    let raw_query = query.into_inner().q;
    let parsed = parse::parse(raw_query.clone());

    let s_target = parsed.search_target().unwrap_or(SearchTarget::Words);
    let query = make_request(parsed.query.clone(), s_target);

    let suggestions = get_suggestions(query)?;

    Ok(gen_output(suggestions, raw_query))
}

fn get_suggestions(query: Request) -> Result<Vec<String>, actix_web::Error> {
    let s_target = query.search_target;
    let res = super::suggestion_ep_inner(query)?
        .suggestions
        .iter()
        .map(|i| {
            let mut s = i.secondary_preferred().to_string();
            if s_target != SearchTarget::Words {
                s.push_str(&format!(" #{s_target:?}"));
            }
            s
        })
        .collect::<Vec<_>>();
    Ok(res)
}

fn gen_output(suggestions: Vec<String>, raw_query: String) -> String {
    let mut data =
        serde_json::to_string(&[suggestions, vec![], vec![]]).unwrap_or_else(|_| "".to_string());
    if data.len() > 2 {
        data = data[1..(data.len() - 1)].to_string();
    }
    format!("[\"{raw_query}\",{data}]")
}

fn make_request(inp: String, search_target: SearchTarget) -> Request {
    Request {
        input: inp,
        lang: "en".to_string(),
        search_target,
        radicals: vec![],
        hashtag: false,
    }
}
