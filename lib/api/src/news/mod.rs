use serde::Serialize;

pub mod detailed;
pub mod short;

#[derive(Serialize, Clone)]
pub struct NewsEntry {
    id: u32,
    title: String,
    html: String,
    creation_time: u64,
    trimmed: bool,
}

impl NewsEntry {
    #[inline]
    fn from_resource(src: &resources::news::NewsEntry, short: bool) -> Self {
        let html = if short {
            src.short.clone()
        } else {
            src.long.clone()
        };

        Self {
            id: src.id,
            html,
            title: src.title.clone(),
            creation_time: src.creation_time,
            trimmed: src.was_trimmed && !short,
        }
    }
}
