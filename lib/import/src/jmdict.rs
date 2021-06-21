use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
};

use deadpool_postgres::Pool;
use itertools::Itertools;
use japanese;
use models::{
    dict::{self, NewDict},
    kanji,
    sense::{self, NewSense},
};
use parse::{jmdict::Parser as jmdictParser, parser::Parse};

struct Word {
    dict: Vec<NewDict>,
    sense: Vec<NewSense>,
}

/// Import jmdict file
pub async fn import(db: Pool, path: String) {
    println!("Clearing existing entries");
    dict::clear_dicts(&db).await.unwrap();
    sense::clear_senses(&db).await.unwrap();

    let path = Path::new(&path);
    let parser = jmdictParser::new(BufReader::new(File::open(path).unwrap()));

    let amount = jmdictParser::new(BufReader::new(File::open(path).unwrap()))
        .count()
        .unwrap();

    println!("Initializing kanji cache");
    kanji::load_kanji_cache(&db).await.unwrap();

    let (sender, receiver): (SyncSender<Word>, Receiver<Word>) = sync_channel(10000);
    let db_clone = db.clone();
    let t1 = std::thread::spawn(move || {
        parser
            .parse(|entry, i| {
                if i % 100 == 0 {
                    print!("\rImporting jmdict... {}%", i * 100 / amount);
                    std::io::stdout().flush().ok();
                }

                let dicts = dict::new_dicts_from_entry(&db_clone, &entry);
                let senses = sense::new_from_entry(&entry);

                sender
                    .send(Word {
                        dict: dicts,
                        sense: senses,
                    })
                    .unwrap();

                false
            })
            .unwrap();
    });

    let mut dicts: Vec<NewDict> = Vec::new();
    let mut senses: Vec<NewSense> = Vec::new();
    let mut received = receiver.recv();

    while received.is_ok() {
        received.iter().for_each(|w| {
            dicts.extend(w.dict.clone());
            senses.extend(w.sense.clone());
        });

        let chunksize = 1000;

        if senses.len() + 400 > chunksize {
            for senses in senses.clone().into_iter().chunks(chunksize).into_iter() {
                sense::insert_sense(&db, senses.collect_vec())
                    .await
                    .unwrap();
            }

            senses.clear();
        }

        if dicts.len() + 400 > chunksize {
            for dicts in dicts.clone().into_iter().chunks(chunksize).into_iter() {
                let mut dicts = dicts.collect_vec();
                get_dict_kanji(&db, &mut dicts).await;
                dict::insert_dicts(&db, dicts).await.unwrap();
            }

            dicts.clear();
        }
        received = receiver.recv();
    }

    sense::insert_sense(&db, senses).await.unwrap();

    get_dict_kanji(&db, &mut dicts).await;
    dict::insert_dicts(&db, dicts).await.unwrap();
    println!();

    t1.join().ok();
}

async fn get_dict_kanji(db: &Pool, dicts: &mut Vec<NewDict>) {
    for dict in dicts.iter_mut() {
        // Skip kana dict-entries
        if !dict.kanji {
            continue;
        }

        let kanji_used = japanese::all_words_with_ct(&dict.reading, japanese::CharType::Kanji);
        let mut db_kanji: Vec<i32> = Vec::new();

        for kanji in kanji_used
            .iter()
            .map(|k| {
                k.chars()
                    .collect_vec()
                    .chunks(1)
                    .map(|i| i.iter().collect::<String>())
                    .collect_vec()
            })
            .flatten()
            .collect_vec()
        {
            let found_kanji = models::kanji::find_by_literalv2(&db, kanji.clone())
                .await
                .ok()
                .and_then(|i| i);

            if found_kanji.is_none() {
                continue;
            }
            let found_kanji = found_kanji.unwrap();

            db_kanji.push(found_kanji.kanji.id);
        }

        if !db_kanji.is_empty() {
            dict.kanji_info = Some(db_kanji);
        }
    }
}
