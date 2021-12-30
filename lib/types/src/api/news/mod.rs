pub mod long;
pub mod short;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NewsEntry {
    pub id: u32,
    pub title: String,
    pub html: String,
    pub creation_time: u64,
    pub trimmed: bool,
}
