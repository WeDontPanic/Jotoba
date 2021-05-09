use crate::parse::jmnedict::Parser as jmnedictParser;
use crate::parse::parser::Parse;
use crate::{models::name, DbPool};
use itertools::Itertools;
use name::NewName;
use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
};

/// Imports jmnedict into database
pub async fn import(db: &DbPool, path: &str) {
    println!("Clearing existing names");
    name::clear(db).await.unwrap();

    let path = Path::new(&path);
    let parser = jmnedictParser::new(BufReader::new(File::open(path).unwrap()));

    let amount = jmnedictParser::new(BufReader::new(File::open(path).unwrap()))
        .count()
        .unwrap();

    let (sender, receiver): (SyncSender<NewName>, Receiver<NewName>) = sync_channel(1000);
    let t1 = std::thread::spawn(move || {
        parser
            .parse(|entry, i| {
                if i % 100 == 0 {
                    print!("\rImporting jmnedict... {}", i * 100 / amount);
                    std::io::stdout().flush().ok();
                }

                sender.send(entry.into()).unwrap();

                false
            })
            .unwrap();
    });

    let mut rec_names: Vec<NewName> = Vec::new();
    let mut received = receiver.recv();

    while received.is_ok() {
        rec_names.push(received.unwrap());

        let chunksize = 10000;

        if rec_names.len() + 400 > chunksize {
            for names in rec_names.clone().into_iter().chunks(chunksize).into_iter() {
                name::insert_names(db, names.collect_vec()).await.unwrap();
            }

            rec_names.clear();
        }
        received = receiver.recv();
    }

    // Insert rest
    name::insert_names(db, rec_names).await.unwrap();
    println!();

    t1.join().ok();
}
