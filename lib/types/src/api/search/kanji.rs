use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub kanji: Vec<Kanji>,
}

#[derive(Serialize, Deserialize)]
pub struct Kanji {
    literal: String,
    meanings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grade: Option<u8>,
    stroke_count: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jlpt: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    onyomi: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kunyomi: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chinese: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    korean_r: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    korean_h: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parts: Option<Vec<String>>,
    radical: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_frames: Option<String>,
}

impl From<&crate::jotoba::kanji::Kanji> for Kanji {
    #[inline]
    fn from(kanji: &crate::jotoba::kanji::Kanji) -> Self {
        let frames = kanji
            .has_stroke_frames()
            .then(|| kanji.get_stroke_frames_url());

        Self {
            literal: kanji.literal.to_string(),
            meanings: kanji.meanings.clone(),
            grade: kanji.grade,
            stroke_count: kanji.stroke_count,
            frequency: kanji.frequency,
            jlpt: kanji.jlpt,
            variant: kanji.variant.clone(),
            onyomi: kanji.onyomi.clone(),
            kunyomi: kanji.kunyomi.clone(),
            chinese: kanji.chinese.clone(),
            korean_r: kanji.korean_r.clone(),
            korean_h: kanji.korean_h.clone(),
            parts: kanji
                .parts
                .as_ref()
                .map(|i| i.iter().map(|i| i.to_string()).collect()),
            radical: kanji.radical.literal.to_string(),
            stroke_frames: frames,
        }
    }
}
