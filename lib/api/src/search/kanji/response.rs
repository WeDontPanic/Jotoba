use search::kanji::result;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Response {
    kanji: Vec<Kanji>,
}

#[derive(Debug, Serialize, Default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    radical: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_frames: Option<String>,
}

impl From<&resources::models::kanji::Kanji> for Kanji {
    fn from(kanji: &resources::models::kanji::Kanji) -> Self {
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
            parts: None,
            radical: None,
            stroke_frames: None,
        }
    }
}

/*
impl From<result::Item> for Kanji {
    fn from(item: result::Item) -> Self {
        Self {
            parts: item.parts.clone(),
            radical: Some(item.radical.literal),
            stroke_frames: Some(item.kanji.kanji.get_stroke_frames_url()),
            ..(&item.kanji).into()
        }
    }
}

impl From<Vec<result::Item>> for Response {
    fn from(items: Vec<result::Item>) -> Self {
        let kanji = items.into_iter().map(|i| Kanji::from(i)).collect();
        Response { kanji }
    }
}
*/
