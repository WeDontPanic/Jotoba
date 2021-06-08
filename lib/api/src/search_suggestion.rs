use std::{sync::Arc, time::SystemTime};

use actix_web::web::{self, Json};
use error::api_error::RestError;
use search::{query::QueryLang, query_parser};
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

/// Request struct for suggestion endpoint
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SuggestionRequest {
    pub input: String,
}

/// Response struct for suggestion endpoint
#[derive(Clone, Debug, Serialize, Default)]
pub struct SuggestionResponse {
    pub suggesstions: Vec<WordPair>,
}

/// a Word with kana and kanji if available
#[derive(Clone, Debug, Serialize, Default)]
pub struct WordPair {
    pub kana: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<String>,
}

/// Get search suggestions
pub async fn suggestion(
    pool: web::Data<Arc<Client>>,
    payload: Json<SuggestionRequest>,
) -> Result<Json<SuggestionResponse>, actix_web::Error> {
    if payload.input.is_empty() {
        return Err(RestError::BadRequest.into());
    }

    let language = query_parser::parse_language(&payload.input);

    let start = SystemTime::now();

    // Get sugesstions for matching language
    let result = match language {
        QueryLang::Japanese => japanese::suggestions(&pool, &payload).await?,
        QueryLang::Foreign | QueryLang::Undetected => foreign::suggestions(&pool, &payload).await?,
    };

    println!("suggestion took: {:?}", start.elapsed());

    Ok(Json(SuggestionResponse {
        suggesstions: result,
    }))
}

mod japanese {
    use std::{iter::FromIterator, time::Duration};

    use actix_web::rt::time::timeout;
    use futures::{stream::FuturesOrdered, TryStreamExt};
    use itertools::Itertools;

    use super::*;

    /// Get suggestions for foreign search input
    pub(super) async fn suggestions(
        client: &Client,
        request: &SuggestionRequest,
    ) -> Result<Vec<WordPair>, RestError> {
        let result = timeout(
            Duration::from_millis(100),
            get_sequence_ids(client, &request.input),
        )
        .await
        .map_err(|_| RestError::Timeout)??;
        Ok(result)
    }

    async fn get_sequence_ids(
        client: &Client,
        query_str: &str,
    ) -> Result<Vec<WordPair>, RestError> {
        let seq_query = "SELECT sequence FROM dict WHERE reading LIKE $1 ORDER BY jlpt_lvl DESC NULLS LAST, ARRAY_LENGTH(priorities,1) DESC NULLS LAST LIMIT 10";

        let rows = client
            .query(seq_query, &[&format!("{}%", query_str).as_str()])
            .await?;

        let mut sequences: Vec<i32> = rows.into_iter().map(|i| i.get(0)).collect();
        sequences.dedup();

        Ok(load_words(&client, &sequences).await?)
    }

    async fn load_words(client: &Client, sequences: &[i32]) -> Result<Vec<WordPair>, RestError> {
        let word_query =
            "select reading, kanji from dict where sequence = $1 and (is_main or kanji = false)";

        let prepared = client.prepare(word_query).await?;

        Ok(FuturesOrdered::from_iter(sequences.into_iter().map(|i| {
            let cloned = prepared.clone();
            async move { client.query(&cloned, &[&i]).await }
        }))
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .filter_map(|word| {
            let words = word
                .into_iter()
                .map(|i| {
                    let reading: String = i.get(0);
                    let is_kanji: bool = i.get(1);
                    (reading, is_kanji)
                })
                .collect_vec();

            let kana = words.iter().find(|i| !i.1)?.0.to_owned();
            let kanji = words.iter().find(|i| i.1).map(|i| i.0.to_owned());

            Some(WordPair { kana, kanji })
        })
        .collect())
    }
}

mod foreign {
    use super::*;

    /// Get suggestions for foreign search input
    pub async fn suggestions(
        pool: &Client,
        request: &SuggestionRequest,
    ) -> Result<Vec<WordPair>, RestError> {
        Ok(vec![])
    }
}
