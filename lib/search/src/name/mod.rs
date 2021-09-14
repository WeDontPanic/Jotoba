mod order;
pub mod result;

use self::result::NameResult;

use super::query::Query;
use error::Error;

use japanese::JapaneseExt;
use resources::models::names::Name;
use utils::to_option;

/// Search for names
pub async fn search(query: &Query) -> Result<NameResult, Error> {
    let res = if query.form.is_kanji_reading() {
        search_kanji(&query).await?
    } else if query.query.is_japanese() {
        search_native(&query).await?
    } else {
        search_transcription(&query).await?
    };

    Ok(NameResult {
        items: res.0,
        total_count: res.1 as u32,
    })
}

/// Search by transcription
async fn search_transcription(query: &Query) -> Result<(Vec<Name>, usize), Error> {
    unimplemented!()
}

/// Search by japanese input
async fn search_native(query: &Query) -> Result<(Vec<Name>, usize), Error> {
    let resources = resources::get().names();

    use crate::engine::name::japanese::Find;

    let res = Find::new(&query.query, 1000, 0).find().await?;

    let len = res.len();

    let names = res
        .retrieve_ordered(|seq_id| resources.by_sequence(seq_id as u32))
        .skip(query.page_offset)
        .take(query.settings.items_per_page as usize)
        .cloned()
        .collect();

    Ok((names, len))
}

/// Search by kanji reading
async fn search_kanji(query: &Query) -> Result<(Vec<Name>, usize), Error> {
    let kanji_reading = query.form.as_kanji_reading().ok_or(Error::Unexpected)?;
    let resources = resources::get().names();

    use crate::engine::name::japanese::Find;

    let res = Find::new(&kanji_reading.literal.to_string(), 1000, 0)
        .find()
        .await?;

    let len = res.len();

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

    Ok((names, len))
}
