use actix_web::web::{self, Json};
use models::DbPool;
use serde::{Deserialize, Serialize};

/// Request struct for kanji_by_radicals endpoint
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SuggestionRequest {
    pub radicals: Vec<char>,
}

/// Response struct for kanji_by_radicals endpoint
#[derive(Clone, Debug, Serialize, Default)]
pub struct SuggestionResponse {
    pub sugesstions: Vec<String>,
}

/// Get kanji by its radicals
pub async fn suggestion(
    pool: web::Data<DbPool>,
    payload: Json<SuggestionRequest>,
) -> Result<Json<SuggestionResponse>, actix_web::Error> {
    Ok(Json(SuggestionResponse {
        sugesstions: Vec::new(),
    }))
}
