mod order;
pub mod result;

use std::time::Instant;

use crate::engine_v2::{self, SerachTask};

use self::result::NameResult;

use super::query::Query;
use error::Error;

use japanese::JapaneseExt;
use utils::to_option;

/// Search for names
#[inline]
pub async fn search(query: &Query) -> Result<NameResult, Error> {
    if query.query.is_japanese() {
        return do_jp(query);
    }

    if query.form.is_kanji_reading() {
        search_kanji(&query).await
    } else {
        do_search(query).await
    }
}

fn do_jp(query: &Query) -> Result<NameResult, Error> {
    let start = Instant::now();

    let search_task: SerachTask<engine_v2::names::native::NativeEngine> =
        SerachTask::new(&query.query)
            .threshold(0f32)
            .offset(query.page_offset)
            .limit(query.settings.items_per_page as usize);

    let (res, len) = search_task.find()?;
    let res: Vec<_> = res
        .into_iter()
        /*
        .skip(query.page_offset)
        .take(query.settings.items_per_page as usize)
        */
        .map(|i| i.item.clone())
        .collect();

    println!("search took: {:?}", start.elapsed());

    Ok(NameResult {
        total_count: len as u32,
        items: res,
    })
}

/// Do a name search
async fn do_search(query: &Query) -> Result<NameResult, Error> {
    use crate::engine::name::{foreign, japanese};

    let start = Instant::now();

    let res = if query.query.is_japanese() {
        japanese::Find::new(&query.query, 1000, 0).find().await?
    } else {
        foreign::Find::new(&query.query, 1000, 0).find().await?
    };

    let resources = resources::get().names();

    let names = res
        .retrieve_ordered(|seq_id| resources.by_sequence(seq_id as u32))
        .skip(query.page_offset)
        .take(query.settings.items_per_page as usize)
        .cloned()
        .collect();

    println!("search took: {:?}", start.elapsed());

    Ok(NameResult {
        items: names,
        total_count: res.len() as u32,
    })
}

/// Search by kanji reading
async fn search_kanji(query: &Query) -> Result<NameResult, Error> {
    let kanji_reading = query.form.as_kanji_reading().ok_or(Error::Unexpected)?;
    let resources = resources::get().names();

    use crate::engine::name::japanese::Find;

    let res = Find::new(&kanji_reading.literal.to_string(), 1000, 0)
        .find()
        .await?;

    let names = res
        .retrieve_ordered(|seq_id| resources.by_sequence(seq_id as u32))
        .filter(|name| {
            if name.kanji.is_none() {
                return false;
            }
            let kanji = name.kanji.as_ref().unwrap();
            let kana = &name.kana;
            let readings = japanese::furigana::generate::retrieve_readings(
                &mut |i: String| {
                    let retrieve = resources::get().kanji();
                    let kanji = retrieve.by_literal(i.chars().next()?)?;
                    if kanji.onyomi.is_none() && kanji.kunyomi.is_none() {
                        return None;
                    }

                    let kun = kanji
                        .clone()
                        .kunyomi
                        .unwrap_or_default()
                        .into_iter()
                        .chain(kanji.natori.clone().unwrap_or_default().into_iter())
                        .collect::<Vec<_>>();
                    let kun = to_option(kun);

                    Some((kun, kanji.onyomi.clone()))
                },
                kanji,
                kana,
            );
            if readings.is_none() {
                return false;
            }

            readings.unwrap().iter().any(|i| {
                i.0.contains(&kanji_reading.literal.to_string())
                    && i.1.contains(&kanji_reading.reading)
            })
        })
        .skip(query.page_offset)
        .take(query.settings.items_per_page as usize)
        .cloned()
        .collect();

    Ok(NameResult {
        items: names,
        total_count: res.len() as u32,
    })
}
