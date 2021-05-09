use std::collections::HashMap;

use actix_web::web::{self, Json};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tokio_diesel::AsyncRunQueryDsl;

use super::error::{Origin, RestError};
use crate::{cache::SharedCache, japanese::JapaneseExt, search::utils::remove_dups, DbPool};
use async_std::sync::Mutex;
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text};
use once_cell::sync::Lazy;

/// Max radicals to allow per request
const MAX_REQUEST_RADICALS: usize = 12;

/// An in memory Cache for kanji items
static RADICAL_CACHE: Lazy<Mutex<SharedCache<Vec<char>, RadicalsResponse>>> =
    Lazy::new(|| Mutex::new(SharedCache::with_capacity(5000)));

/// Request struct for kanji_by_radicals endpoint
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RadicalsRequest {
    pub radicals: Vec<char>,
}

/// Response struct for kanji_by_radicals endpoint
#[derive(Clone, Debug, Serialize, Default)]
pub struct RadicalsResponse {
    pub kanji: HashMap<i32, Vec<char>>,
    pub possible_radicals: Vec<char>,
}

/// Get kanji by its radicals
pub async fn kanji_by_radicals(
    pool: web::Data<DbPool>,
    payload: Json<RadicalsRequest>,
) -> Result<Json<RadicalsResponse>, actix_web::Error> {
    // Validate an adjust request
    let payload = validate_request(&payload)?;

    // Try to use cache
    if let Some(cached) = get_cache(&payload.radicals).await {
        return Ok(Json(cached));
    }

    // Kanji by search-radicals from DB
    let kanji = find_by_radicals(&pool, &payload.radicals).await?;

    // IDs of [`kanji`]
    let kanji_ids = kanji.iter().map(|i| i.id).collect_vec();

    // Build a new response
    let response = RadicalsResponse {
        kanji: format_kanji(&kanji),
        possible_radicals: posible_radicals(&pool, &kanji_ids, &payload.radicals).await?,
    };

    set_cache(payload.radicals, response.clone()).await;

    Ok(Json(response))
}

/// Finds all kanji which are constructed used the passing radicals
async fn find_by_radicals(db: &DbPool, radicals: &[char]) -> Result<Vec<SqlFindResult>, RestError> {
    let mut rad_where: Vec<String> = vec![];

    for radical in radicals {
        rad_where.push(format!(" r.literal = '{}' ", radical));
    }

    let query = format!(
        "SELECT id, literal, stroke_count FROM kanji WHERE kanji.id IN 
            (SELECT k.kanji_id FROM kanji_element AS k 
                JOIN search_radical AS r on r.id = k.search_radical_id 
                WHERE {} GROUP BY k.kanji_id
                HAVING COUNT(*) >= {}) order by stroke_count, grade",
        rad_where.join("OR"),
        radicals.len(),
    );

    Ok(diesel::sql_query(query).get_results_async(db).await?)
}

/// Returns a vec of all possible radicals
async fn posible_radicals(
    db: &DbPool,
    kanji_ids: &[i32],
    radicals: &[char],
) -> Result<Vec<char>, RestError> {
    use crate::schema::kanji_element;
    use crate::schema::search_radical::dsl::*;

    Ok(search_radical
        .select(literal)
        .distinct()
        .inner_join(kanji_element::table)
        .filter(kanji_element::kanji_id.eq_any(kanji_ids))
        .get_results_async::<String>(db)
        .await?
        .into_iter()
        // Get char from string
        .map(|i| i.chars().next().unwrap())
        // Skip all already provided radicals
        .filter(|i| !radicals.contains(i))
        .collect())
}

#[derive(QueryableByName, Debug)]
struct SqlFindResult {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub literal: String,
    #[sql_type = "Integer"]
    pub stroke_count: i32,
}

/// Validates the kanji by radicals request
pub fn validate_request(payload: &RadicalsRequest) -> Result<RadicalsRequest, RestError> {
    // filter out all non radicals
    let radicals = payload
        .radicals
        .iter()
        .filter(|i| i.is_radical())
        .copied()
        .collect::<Vec<_>>();

    if radicals.is_empty() {
        return Err(RestError::Missing(Origin::Radicals));
    }

    if radicals.len() > MAX_REQUEST_RADICALS {
        return Err(RestError::BadRequest);
    }

    let mut actual_radicals = remove_dups(radicals)
        .into_iter()
        .filter(|i| i.is_radical())
        .collect_vec();

    // Sort radicals because we need to check them against cache. The same result should be
    // returned if the input radicals are the same but in a different order
    actual_radicals.sort();

    // Adjust request
    Ok(RadicalsRequest {
        radicals: actual_radicals,
    })
}

/// Formats the kanji result and places each item into a hashmap with the stroke_count as key
fn format_kanji(sql_result: &Vec<SqlFindResult>) -> HashMap<i32, Vec<char>> {
    let mut kanji_map = HashMap::new();
    for kanji in sql_result.iter() {
        kanji_map
            .entry(kanji.stroke_count)
            .or_insert(vec![])
            .push(kanji.literal.chars().next().unwrap())
    }
    kanji_map
}

async fn get_cache(payload: &Vec<char>) -> Option<RadicalsResponse> {
    RADICAL_CACHE
        .lock()
        .await
        .cache_get(payload)
        .map(|i| i.to_owned())
}

async fn set_cache(payload: Vec<char>, response: RadicalsResponse) {
    RADICAL_CACHE.lock().await.cache_set(payload, response);
}
