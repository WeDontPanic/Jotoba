use models::{dict, search_mode::SearchMode, sense, DbPool};
use parse::jmdict::{
    languages::Language, part_of_speech::PartOfSpeech, Entry, EntryElement, EntrySense, GlossValue,
};
use search::word::WordSearch;
use serde_json::Value;

use futures::future::try_join_all;

/// Import manga sfx items file
pub async fn import(db: &DbPool, path: &str) {
    println!("Importing sfx patches...");
    let f = std::fs::File::open(path).expect("Error reading jlpt patch file!");
    let json: Value = serde_json::from_reader(f).expect("invalid json data");
    let json = json.as_object().unwrap();

    let mut min_seq = dict::min_sequence(&db).await.expect("db err");

    try_join_all(json.into_iter().filter_map(|(sfx_item, translation)| {
        if let Value::String(translation) = translation {
            let seq = min_seq - 100;
            min_seq = seq;
            Some(import_sfx(db, seq, sfx_item.to_owned(), translation))
        } else {
            None
        }
    }))
    .await
    .expect("db error");
}

/// Import a single sfx item
async fn import_sfx(
    db: &DbPool,
    seq: i32,
    jp: String,
    translation: &String,
) -> Result<(), error::Error> {
    // TODO also search for katakana (or hiragana) version to prevent cross-kana duplicates
    let native = WordSearch::new(db, &jp)
        .with_mode(SearchMode::Exact)
        .with_kana_only(true)
        .search_native(|_| ())
        .await
        .unwrap()
        .0;

    let seq = {
        if native.is_empty() {
            seq
        } else {
            native[0].sequence
        }
    };

    let entry = Entry {
        elements: vec![EntryElement {
            value: jp,
            ..Default::default()
        }],
        senses: vec![EntrySense {
            lang: Language::English,
            glosses: vec![GlossValue {
                value: translation.to_owned(),
                language: Language::English,
                g_type: None,
            }],
            part_of_speech: vec![PartOfSpeech::Sfx],
            ..Default::default()
        }],
        sequence: seq as u64,
    };

    let db_connection = db.get().unwrap();
    let dicts = dict::new_dicts_from_entry(&db_connection, &entry);
    let senses = sense::new_from_entry(&entry);

    sense::insert_sense(&db, senses).await?;

    if native.is_empty() {
        dict::insert_dicts(db, dicts).await?;
    }

    Ok(())
}
