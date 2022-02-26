/// A displayable part of a "sentence"
#[derive(Clone, Debug, PartialEq)]
pub struct SentencePart {
    pub text: String,
    pub info: Option<&'static str>,
    pub furigana: Option<String>,
    pub furi_guessed: bool,
    pub pos: i32,
    pub add_class: Option<String>,
    pub lexeme: String,
}

impl SentencePart {
    pub fn get_add_class(&self) -> String {
        self.add_class.as_ref().cloned().unwrap_or_default()
    }
}
