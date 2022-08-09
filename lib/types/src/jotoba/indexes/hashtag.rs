use std::str::FromStr;

use crate::jotoba::search::SearchTarget;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RawHashtag {
    pub tag: String,
    pub s_targets: Vec<SearchTarget>,
    pub freq: f32,
}

impl RawHashtag {
    pub fn new(tag: String, s_targets: Vec<SearchTarget>, freq: f32) -> Self {
        Self {
            tag,
            s_targets,
            freq,
        }
    }
}

impl FromStr for RawHashtag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }

        let mut split = s.trim().split(' ');
        let tag = split.next().ok_or(())?.to_string();
        let freq = split.next().and_then(|i| i.parse::<f32>().ok()).ok_or(())?;
        let s_targets = split
            .map(|o| {
                o.parse::<u8>()
                    .ok()
                    .and_then(|i| SearchTarget::try_from(i).ok())
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Ok(RawHashtag::new(tag, s_targets, freq))
    }
}
