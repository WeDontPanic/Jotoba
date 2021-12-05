/// An kanji character. Represents one Kanji
#[derive(Default, Clone, Debug)]
pub struct Character {
    pub literal: char,
    pub on_readings: Vec<String>,
    pub kun_readings: Vec<String>,
    pub chinese_readings: Vec<String>,
    pub korean_romanized: Vec<String>,
    pub korean_hangul: Vec<String>,
    pub meaning: Vec<String>,
    pub grade: Option<u8>,
    pub stroke_count: u8,
    pub variant: Vec<String>,
    pub frequency: Option<u16>,
    pub jlpt: Option<u8>,
    pub natori: Vec<String>,
    pub radical: Option<i32>,
}
