use tokio_postgres::Row;

use super::*;

/// Reading response (onyomi, kunyomi)
pub(crate) struct ReadingsRes(Vec<String>, Vec<String>);

/// Gets suggestions for kanji reading search eg: "痛 いた.い"
pub(crate) async fn suggestions(
    client: &Client,
    kanji_reading: KanjiReading,
) -> Result<Response, RestError> {
    let literal = kanji_reading.literal;
    let reading = kanji_reading.reading.replace("。", "").replace(".", "");

    // Find readings
    let mut readings = readings_by_lit(client, literal)
        .await?
        .get_suggestions(literal);

    // User provided additionally a part of a reading
    if !reading.is_empty() {
        readings.sort_by(|a, b| order(a, b, &reading));
    }

    Ok(Response {
        suggestions: readings,
        suggestion_type: SuggestionType::KanjiReading,
    })
}

fn order(a: &WordPair, b: &WordPair, reading: &str) -> Ordering {
    utils::bool_ord(starts_with(a, reading), starts_with(b, reading))
}

fn starts_with(word: &WordPair, reading: &str) -> bool {
    word.primary.replace(".", "").starts_with(&reading)
}

impl ReadingsRes {
    /// Returns all suggestions of the [`KanjiRes`]
    fn get_suggestions(self, literal: char) -> Vec<WordPair> {
        Self::to_word_pair(self.0, literal)
            .chain(Self::to_word_pair(self.1, literal))
            .collect()
    }

    /// Returns an iterator over word paris with the passed readings
    fn to_word_pair(readings: Vec<String>, literal: char) -> impl Iterator<Item = WordPair> {
        readings.into_iter().map(move |i| WordPair {
            primary: i,
            secondary: Some(literal.to_string()),
        })
    }
}

impl From<Row> for ReadingsRes {
    fn from(row: Row) -> Self {
        Self {
            0: row.get(0),
            1: row.get(1),
        }
    }
}

/// Returns a single item of [`ReadingsRes`] for the kanji identified by its literal
async fn readings_by_lit(client: &Client, literal: char) -> Result<ReadingsRes, RestError> {
    let query = "SELECT onyomi, kunyomi FROM kanji WHERE literal = $1 LIMIT 1";

    let statement = client.prepare(query).await?;
    let res: ReadingsRes = client
        .query_one(&statement, &[&literal.to_string()])
        .await?
        .into();

    Ok(res)
}
