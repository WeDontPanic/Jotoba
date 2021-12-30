use serde::{Deserialize, Serialize};

use super::NewsEntry;

#[derive(Deserialize)]
pub struct Request {
    pub after: u64,
}

#[derive(Serialize)]
pub struct Response {
    pub entries: Vec<NewsEntry>,
}
