use std::str::FromStr;

use crate::DbPool;
use crate::{models::sentence, parse::jmdict::languages::Language};
use itertools::Itertools;
use serde_json::Value;

/// Import jlpt patche file
pub async fn import(db: &DbPool, path: String) {
    println!("Clearing old sentences");
    sentence::clear(db).await.unwrap();
    let f = std::fs::File::open(&path).expect("Error reading sentences file!");
    println!("Counting sentences...");
    let json: Value = serde_json::from_reader(f).expect("invalid json data");
    let len = json.get("sentences").unwrap().as_array().unwrap().len();

    println!("Importing sentences...");
    let f = std::fs::File::open(path).expect("Error reading sentences file!");
    let json: Value = serde_json::from_reader(f).expect("invalid json data");

    // Import kanji patches
    if let Some(sentences) = json.get("sentences").and_then(|i| i.as_array()) {
        for (pos, sentence) in sentences.into_iter().enumerate() {
            if let Some(sentence_object) = sentence.as_object() {
                let jp = sentence_object.get("jp").and_then(|i| i.as_str());
                let translations = sentence_object.get("translated").and_then(|i| i.as_array());
                if jp.is_none() || translations.is_none() {
                    continue;
                }
                let jp = jp.unwrap();
                let furigana = sentence_object
                    .get("furi")
                    .and_then(|i| i.as_str())
                    .unwrap()
                    .to_owned();
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

                sentence::insert_sentence(&db, jp.to_owned(), furigana.to_owned(), translations)
                    .await
                    .expect("err");
                print!("Imported {}/{}\r", pos, len);
            }
        }
    }
}
