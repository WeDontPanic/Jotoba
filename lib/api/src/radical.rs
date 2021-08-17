use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    _payload: Json<RadicalsRequest>,
) -> Result<Json<RadicalsResponse>, actix_web::Error> {
    /*
    // Validate an adjust request
    let payload = validate_request(&payload)?;

    // Try to use cache
    if let Some(cached) = get_cache(&payload.radicals).await {
        return Ok(Json(cached));
    }

    // Kanji by search-radicals from DB
    let kanji = find_by_radicals(&payload.radicals).await?;

    // IDs of [`kanji`]
    let kanji_ids = kanji.iter().map(|i| i.id).collect_vec();

    // Build a new response
    let response = RadicalsResponse {
        kanji: format_kanji(&kanji),
        possible_radicals: posible_radicals(&poolv2, &kanji_ids, &payload.radicals).await?,
    };

    set_cache(payload.radicals, response.clone()).await;
    */

    Ok(Json(RadicalsResponse::default()))
}

/*
/// Finds all kanji which are constructed used the passing radicals
async fn find_by_radicals(pool: &Pool, radicals: &[char]) -> Result<Vec<SqlFindResult>, RestError> {
    if radicals.is_empty() {
        return Ok(vec![]);
    }

    let pool = pool.get().await?;
    let prepared = pool.prepare_cached("SELECT kanji_element.kanji_id, search_radical.literal FROM kanji_element
              JOIN search_radical ON search_radical.id = kanji_element.search_radical_id
                WHERE kanji_element.kanji_id in
                  (SELECT kanji_element.kanji_id FROM search_radical JOIN kanji_element
                    ON kanji_element.search_radical_id = search_radical.id where search_radical.literal = $1)").await?;

    let kanji_ids: Vec<SqlKanjiLiteralResult> = pool
        .query(&prepared, &[&radicals[0].to_string()])
        .await?
        .into_iter()
        .map(|i| SqlKanjiLiteralResult::from(i))
        .collect();

    // Kanji ids which have all [`radicals`]
    let kanji_ids = kanji_ids
        .into_iter()
        .group_by(|i| i.kanji_id)
        .into_iter()
        .filter_map(|(k_id, rads)| {
            let rads = rads
                .into_iter()
                .map(|j| j.literal.chars().next().unwrap())
                .collect_vec();

            // Filter all kanji ids of kanji which have all radicals
            part_of(radicals, &rads).then(|| k_id)
        })
        .collect_vec();

    let prepared = pool
        .prepare_cached(
            "SELECT id, literal, stroke_count FROM kanji WHERE id = ANY ($1) ORDER BY stroke_count, grade",
        )
        .await?;

    Ok(pool
        .query(&prepared, &[&kanji_ids])
        .await?
        .into_iter()
        .map(|i| SqlFindResult::from(i))
        .collect())
}

/// Returns a vec of all possible radicals
async fn posible_radicals(
    db: &Pool,
    kanji_ids: &[i32],
    radicals: &[char],
) -> Result<Vec<char>, RestError> {
    let db = db.get().await?;

    let prepared = db.prepare_cached("SELECT DISTINCT literal FROM search_radical INNER JOIN kanji_element ON kanji_element.search_radical_id = search_radical.id WHERE kanji_id = ANY($1)").await?;

    Ok(db
        .query(&prepared, &[&kanji_ids])
        .await?
        .into_iter()
        .filter_map(|i| {
            let s: String = i.get(0);
            s.chars()
                .next()
                .and_then(|j| (!radicals.contains(&j)).then(|| j))
        })
        .collect())
}

struct SqlKanjiLiteralResult {
    pub kanji_id: i32,
    pub literal: String,
}

impl From<Row> for SqlKanjiLiteralResult {
    fn from(row: Row) -> Self {
        Self {
            kanji_id: row.get(0),
            literal: row.get(1),
        }
    }
}

struct SqlFindResult {
    pub id: i32,
    pub literal: String,
    pub stroke_count: i32,
}

impl From<Row> for SqlFindResult {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            literal: row.get(1),
            stroke_count: row.get(2),
        }
    }
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

    if radicals.is_empty() || radicals.len() > MAX_REQUEST_RADICALS {
        return Err(RestError::BadRequest);
    }

    let mut actual_radicals = remove_dups(radicals)
        .into_iter()
        .filter(|i| i.is_radical())
        .collect_vec();

    // Sort radicals because we need to check them against cache. The same result should be
    // returned if the input radicals are the same but in a different order
    actual_radicals.sort_unstable();

    // Adjust request
    Ok(RadicalsRequest {
        radicals: actual_radicals,
    })
}

/// Formats the kanji result and places each item into a hashmap with the stroke_count as key
fn format_kanji(sql_result: &[SqlFindResult]) -> HashMap<i32, Vec<char>> {
    let mut kanji_map = HashMap::new();
    for kanji in sql_result.iter() {
        kanji_map
            .entry(kanji.stroke_count)
            .or_insert_with(Vec::new)
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
*/
