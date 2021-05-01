use std::str::FromStr;

use crate::DbPool;
use crate::{models::sentence, parse::jmdict::languages::Language};
use itertools::Itertools;
use serde_json::Value;

use futures::future::try_join_all;

/// Import jlpt patche file
pub async fn import(db: &DbPool, path: String) {
    println!("Clearing old sentences");
    sentence::clear(db).await.unwrap();
    println!("Importing sentences...");
    let f = std::fs::File::open(path).expect("Error reading sentences file!");
    let json: Value = serde_json::from_reader(f).expect("invalid json data");

    // Import kanji patches
    if let Some(sentences) = json.get("sentences").and_then(|i| i.as_array()) {
        try_join_all(sentences.into_iter().filter_map(|sentence| {
            if let Some(sentence_object) = sentence.as_object() {
                let jp = sentence_object.get("jp").and_then(|i| i.as_str());
                let translations = sentence_object.get("translated").and_then(|i| i.as_array());
                if jp.is_none() || translations.is_none() {
                    return None;
                }
                let jp = jp.unwrap();
                let translations = translations
                    .unwrap()
                    .into_iter()
                    .map(|i| {
                        let obj = i.as_object().unwrap();
                        let text = obj.get("text").and_then(|i| i.as_str()).unwrap().to_owned();
                        let lang = obj.get("language").and_then(|i| i.as_str()).unwrap();
                        let lang = Language::from_str(lang).unwrap();
                        (text, lang)
                    })
                    .collect_vec();

                Some(sentence::insert_sentence(&db, jp.to_owned(), translations))
            } else {
                None
            }
        }))
        .await
        .expect("db error");
    }
}
