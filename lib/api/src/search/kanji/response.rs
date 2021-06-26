use models::kanji::KanjiResult;
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
    grade: Option<i32>,
    stroke_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jlpt: Option<i32>,
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

impl From<&KanjiResult> for Kanji {
    fn from(kanji: &KanjiResult) -> Self {
        Self {
            literal: kanji.kanji.literal.clone(),
            meanings: kanji.meanings.clone(),
            grade: kanji.kanji.grade,
            stroke_count: kanji.kanji.stroke_count,
            frequency: kanji.kanji.frequency,
            jlpt: kanji.kanji.jlpt,
            variant: kanji.kanji.variant.clone(),
            onyomi: kanji.kanji.onyomi.clone(),
            kunyomi: kanji.kanji.kunyomi.clone(),
            chinese: kanji.kanji.chinese.clone(),
            korean_r: kanji.kanji.korean_r.clone(),
            korean_h: kanji.kanji.korean_h.clone(),
            parts: None,
            radical: None,
            stroke_frames: None,
        }
    }
}

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
