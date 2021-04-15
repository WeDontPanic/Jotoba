use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
};

use crate::parse::jmdict::Parser as jmdictParser;
use crate::parse::parser::Parse;
use crate::{
    models::{
        dict::{self, NewDict},
        sense::{self, NewSense},
    },
    DbPool,
};
use itertools::Itertools;

struct Word {
    dict: Vec<NewDict>,
    sense: Vec<NewSense>,
}

pub async fn import(db: &DbPool, path: String) {
    println!("Clearing existing entries");
    dict::clear_dicts(db).await.unwrap();
    sense::clear_senses(db).await.unwrap();

    let path = Path::new(&path);
    let parser = jmdictParser::new(BufReader::new(File::open(path).unwrap()));

    let amount = jmdictParser::new(BufReader::new(File::open(path).unwrap()))
        .count()
        .unwrap();

    let (sender, receiver): (SyncSender<Word>, Receiver<Word>) = sync_channel(1000);
    let t1 = std::thread::spawn(move || {
        parser
            .parse(|entry, i| {
                if i % 100 == 0 {
                    print!("\rImporting jmdict... {}", i * 100 / amount);
                    std::io::stdout().flush().ok();
                }

                let dicts = dict::new_dicts_from_entry(&entry);
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

        let chunksize = 10000;

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
                dict::insert_dicts(db, dicts.collect_vec()).await.unwrap();
            }

            dicts.clear();
        }
        received = receiver.recv();
    }

    sense::insert_sense(db, senses).await.unwrap();
    dict::insert_dicts(db, dicts).await.unwrap();

    t1.join().ok();
}
