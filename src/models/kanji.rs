use std::{fs::read_to_string, path::Path};

use super::super::schema::kanji;
use crate::{error::Error, parse::kanjidict::Character, DbPool};
use diesel::prelude::*;
use tokio_diesel::*;

#[derive(Queryable, Clone, Debug, Default, PartialEq)]
pub struct Kanji {
    pub id: i32,
    pub literal: String,
    pub meaning: Vec<String>,
    pub grade: Option<i32>,
    pub stroke_count: i32,
    pub frequency: Option<i32>,
    pub jlpt: Option<i32>,
    pub variant: Option<Vec<String>>,
    pub onyomi: Option<Vec<String>>,
    pub kunyomi: Option<Vec<String>>,
    pub chinese: Option<String>,
    pub korean_r: Option<Vec<String>>,
    pub korean_h: Option<Vec<String>>,
    pub natori: Option<Vec<String>>,
}

#[derive(Insertable, Clone, Debug, PartialEq, Default)]
#[table_name = "kanji"]
pub struct NewKanji {
    pub literal: String,
    pub meaning: Vec<String>,
    pub grade: Option<i32>,
    pub stroke_count: i32,
    pub frequency: Option<i32>,
    pub jlpt: Option<i32>,
    pub variant: Option<Vec<String>>,
    pub onyomi: Option<Vec<String>>,
    pub kunyomi: Option<Vec<String>>,
    pub chinese: Option<String>,
    pub korean_r: Option<Vec<String>>,
    pub korean_h: Option<Vec<String>>,
    pub natori: Option<Vec<String>>,
}

impl From<Character> for NewKanji {
    fn from(k: Character) -> Self {
        Self {
            literal: k.literal.into(),
            meaning: k.meaning.clone(),
            grade: k.grade,
            stroke_count: k.stroke_count,
            frequency: k.frequency,
            jlpt: k.jlpt,
            variant: to_option(k.variant),
            onyomi: to_option(k.on_readings),
            kunyomi: to_option(k.kun_readings),
            chinese: k.chinese_reading,
            korean_r: to_option(k.korean_romanized),
            korean_h: to_option(k.korean_hangul),
            natori: to_option(k.natori),
        }
    }
}

fn to_option<T>(vec: Vec<T>) -> Option<Vec<T>> {
    if vec.is_empty() {
        None
    } else {
        Some(vec)
    }
}

impl Kanji {
    /// Print kanji grade pretty for frontend
    pub fn school_str(&self) -> Option<String> {
        self.grade.map(|grade| format!("Taught in {} grade", grade))
    }

    pub fn get_animation_path(&self) -> String {
        format!("html/assets/svg/{}_animated.svgs", self.literal)
    }

    pub fn get_stroke_frames_url(&self) -> String {
        format!("assets/svg/{}_frames.svg", self.literal)
    }

    // Returns true if the kanji has a stroke animation file
    pub fn has_animation_file(&self) -> bool {
        Path::new(&self.get_animation_path()).exists()
    }

    // Returns true if the kanji has stroke frames
    pub fn has_stroke_frames(&self) -> bool {
        Path::new(&self.get_animation_path()).exists()
    }

    /// Return the animation entries for the template
    pub fn get_animation_entries(&self) -> Vec<(String, String)> {
        if let Ok(content) = read_to_string(self.get_animation_path()) {
            content
                .split("\n")
                .into_iter()
                .map(|i| {
                    let mut s = i.split(";");
                    (s.next().unwrap().to_owned(), s.next().unwrap().to_owned())
                })
                .collect::<Vec<(String, String)>>()
        } else {
            vec![]
        }
    }
}

/// Inserts new kanji into db
pub async fn insert<T>(db: &DbPool, new_kanji: Vec<T>) -> Result<(), Error>
where
    T: Into<NewKanji>,
{
    use crate::schema::kanji::dsl::*;

    let items: Vec<NewKanji> = new_kanji.into_iter().map(|i| i.into()).collect();

    diesel::insert_into(kanji)
        .values(items)
        .execute_async(db)
        .await?;

    Ok(())
}

/// Clear all kanji entries
pub async fn clear_kanji(db: &DbPool) -> Result<(), Error> {
    use crate::schema::kanji::dsl::*;
    diesel::delete(kanji).execute_async(db).await?;
    Ok(())
}

/// Returns Ok(true) if at least one kanji exists in the Db
pub async fn exists(db: &DbPool) -> Result<bool, Error> {
    use crate::schema::kanji::dsl::*;
    Ok(kanji.select(id).limit(1).execute_async(db).await? == 1)
}

/// Find a kanji by its literal
pub async fn find_by_literal(db: &DbPool, l: &str) -> Result<Kanji, Error> {
    use crate::schema::kanji::dsl::*;
    kanji
        .filter(literal.eq(l))
        .limit(1)
        .get_result_async(db)
        .await
        .map_err(|i| i.into())
}

/// Find Kanji items by its ids
pub async fn load_by_ids(db: &DbPool, ids: &Vec<i32>) -> Result<Vec<Kanji>, Error> {
    use crate::schema::kanji::dsl::*;
    kanji
        .filter(id.eq_any(ids))
        .get_results_async(db)
        .await
        .map_err(|i| i.into())
}
