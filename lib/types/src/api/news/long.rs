use serde::{Deserialize, Serialize};

use super::NewsEntry;

#[derive(Deserialize)]
pub struct Request {
    pub id: u32,
}

#[derive(Serialize)]
pub struct Response {
    pub entry: NewsEntry,
}
