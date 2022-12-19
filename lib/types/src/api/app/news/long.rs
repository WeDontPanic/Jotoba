use serde::{Deserialize, Serialize};

use super::NewsEntry;

#[derive(Deserialize)]
pub struct Request {
    pub id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub entry: NewsEntry,
}
