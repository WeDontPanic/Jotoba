use models::{dict, kanji, DbPool};
use serde_json::Value;

use futures::future::try_join_all;

/// Import jlpt patche file
pub async fn import(db: &DbPool, path: &str) {
    println!("Importing jlpt patches...");
    let f = std::fs::File::open(path).expect("Error reading jlpt patch file!");
    let json: Value = serde_json::from_reader(f).expect("invalid json data");

    // Import kanji patches
    if let Some(kanji_patches) = json.get("kanji").and_then(|i| i.as_object()) {
        try_join_all(kanji_patches.into_iter().filter_map(|(kanji_item, jlpt)| {
            if let Value::Number(nr) = jlpt {
                Some(kanji::update_jlpt(
                    &db,
                    &kanji_item,
                    nr.as_i64().unwrap() as i32,
                ))
            } else {
                None
            }
        }))
        .await
        .expect("db error");
    }

    // Import voc patches
    if let Some(voc_patches) = json.get("vocs").and_then(|i| i.as_object()) {
        try_join_all(voc_patches.into_iter().filter_map(|(voc_item, jlpt)| {
            if let Value::Number(nr) = jlpt {
                Some(dict::update_jlpt(
                    &db,
                    &voc_item,
                    nr.as_i64().unwrap() as i32,
                ))
            } else {
                None
            }
        }))
        .await
        .expect("db error");
    }
}
