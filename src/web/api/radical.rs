use std::collections::HashMap;

use actix_web::web::{self, Json};
use serde::{Deserialize, Serialize};
use tokio_diesel::AsyncRunQueryDsl;

use super::error::{Origin, RestError};
use crate::{japanese::JapaneseExt, search::utils::remove_dups, DbPool};
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text};

/// Max radicals to allow per request
const MAX_REQUEST_RADICALS: usize = 12;

/// Request struct for kanji_by_radicals endpoint
#[derive(Clone, Debug, Deserialize)]
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
    let payload = validate_request(&payload)?;

    let kanji = find_by_radicals(&pool, &payload.radicals).await?;

    let mut kanji_map = HashMap::new();
    for kanji in kanji.iter() {
        kanji_map
            .entry(kanji.stroke_count)
            .or_insert(vec![])
            .push(kanji.literal.chars().next().unwrap())
    }

    let kanji_ids: Vec<i32> = kanji.iter().map(|i| i.id).collect();

    Ok(Json(RadicalsResponse {
        kanji: kanji_map,
        possible_radicals: posible_radicals(&pool, &kanji_ids, &payload.radicals).await?,
    }))
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

    // Adjust request
    Ok(RadicalsRequest {
        radicals: remove_dups(radicals)
            .into_iter()
            .filter(|i| i.is_radical())
            .collect(),
    })
}
