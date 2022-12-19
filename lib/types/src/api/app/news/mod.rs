pub mod long;
pub mod short;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct NewsEntry {
    pub id: u32,
    pub title: String,
    pub html: String,
    pub creation_time: u64,
    pub trimmed: bool,
}
