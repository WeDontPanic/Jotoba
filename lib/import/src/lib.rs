use std::path::Path;

use deadpool_postgres::Pool;
use error::Error;
use models::{dict, kanji, name, radical, sense, DbConnection};

pub mod accents;
pub mod jlpt_patches;
pub mod jmdict;
pub mod jmnedict;
pub mod kanji_elements;
pub mod kanjidict;
pub mod manga_sfx;
pub mod radicals;
pub mod search_radicals;
pub mod sentences;

#[derive(Default)]
pub struct Options {
    pub jmdict_path: String,
    pub kanjidict_path: String,
    pub jlpt_paches_path: String,
    pub manga_sfx_path: String,
    pub jmnedict_path: String,
    pub sentences_path: String,
    pub accents_path: String,
    pub radicals_path: String,
    pub elements_path: String,
    pub search_radicals_path: String,
}

impl Options {
    pub fn get_import_paths(&self) -> Vec<&String> {
        vec![
            &self.jmdict_path,
            &self.kanjidict_path,
            &self.jlpt_paches_path,
            &self.manga_sfx_path,
            &self.jmnedict_path,
            &self.sentences_path,
            &self.accents_path,
            &self.radicals_path,
            &self.elements_path,
            &self.search_radicals_path,
        ]
        .into_iter()
        .filter(|i| !i.is_empty())
        .collect()
    }

    pub fn has_import_data(&self) -> bool {
        !self.get_import_paths().is_empty()
    }

    pub fn paths_exists(&self) -> bool {
        let paths = self.get_import_paths().into_iter().map(|i| Path::new(i));

        for path in paths {
            if !path.exists() {
                println!("Path '{}' not found", path.display());
                return false;
            }
        }

        true
    }
}

/// Returns true if DB has all required data
pub async fn has_required_data(pool: &Pool) -> Result<bool, Error> {
    let jmdict_exists = dict::exists(&pool).await? && sense::exists(&pool).await?;
    let jmnedict_exists = name::exists(&pool).await?;
    let kanji_exists = kanji::exists(&pool).await?;

    let radicals_exists = radical::exists(&pool).await?;
    let search_radicals_exists = radical::search_radical_exists(&pool).await?;
    let kanji_elements_exists = kanji::element_exists(&pool).await?;

    if !jmdict_exists {
        println!("Jmdict missing");
    }

    if !jmnedict_exists {
        println!("Jmnedict missing");
    }

    if !kanji_exists {
        println!("Kanji missing");
    }

    if !radicals_exists {
        println!("Radicals missing");
    }

    if !search_radicals_exists {
        println!("Search radicals missing");
    }

    if !kanji_elements_exists {
        println!("Kradfile missing");
    }

    Ok(jmdict_exists
        && jmnedict_exists
        && kanji_exists
        && radicals_exists
        && kanji_elements_exists
        && search_radicals_exists)
}

/// Import data
pub async fn import(database: &DbConnection, pool: &Pool, options: &Options) {
    if !options.has_import_data() {
        println!("No import files were provided!");
        return;
    }

    if !options.paths_exists() {
        return;
    }

    // Import all independent items first
    import_independent(database, pool, options).await;

    let kanji_exists = kanji::exists(&pool).await.expect("fatal db err");
    let jmdict_exists = dict::exists(&pool).await.expect("fatal DB error")
        && sense::exists(&pool).await.expect("fatal db error");
    let mut imported_dicts = false;

    // From here on we're depending on kanji elements
    if !kanji_exists {
        println!("Kanji missing. Import the kanjidict first!");
        return;
    }

    // Import kanji elements
    if !options.elements_path.is_empty() {
        // TODO Check if search radicals exists
        kanji_elements::import(&database, &options.elements_path).await;
    }

    // Import JMDict
    if !options.jmdict_path.is_empty() {
        jmdict::import(&database, options.jmdict_path.clone()).await;
        imported_dicts = true;
    }

    // Update kun readings for kanji
    if (!options.kanjidict_path.is_empty() && (jmdict_exists || imported_dicts))
        || (imported_dicts && (!options.kanjidict_path.is_empty() || kanji_exists))
    {
        update_dict_links(database).await.expect("Fatal DB error");
    }

    // From here on we're depending on JMDict elements
    if !jmdict_exists && !imported_dicts {
        println!("You need to import JMDict first!");
        return;
    }

    // JLPT patches
    if !options.jlpt_paches_path.is_empty() {
        jlpt_patches::import(&database, &options.jlpt_paches_path).await;
    }

    // Accents
    if !options.accents_path.is_empty() {
        accents::import(&database, &options.accents_path).await;
    }

    // Manga patches
    if !options.manga_sfx_path.is_empty() {
        manga_sfx::import(&database, &options.manga_sfx_path).await;
    }

    // Sentences
    if !options.sentences_path.is_empty() {
        sentences::import(&database, &options.sentences_path).await;
    }
}

/// Updates Kun and On readings for kanji
pub async fn update_dict_links(database: &DbConnection) -> Result<(), Error> {
    kanji::gen_readings::update_links(&database).await?;
    dict::collocations::generate(&database).await?;
    Ok(())
}

/// Import independent items
async fn import_independent(database: &DbConnection, pool: &Pool, options: &Options) {
    // Kanji dict
    if !options.kanjidict_path.is_empty() {
        kanjidict::import(&database, options.kanjidict_path.clone()).await;
    }

    // Radicals
    if !options.radicals_path.is_empty() {
        radicals::import(&database, &options.radicals_path).await;
    }

    // Search radicals
    if !options.search_radicals_path.is_empty() {
        search_radicals::import(&database, &options.search_radicals_path).await;
    }

    // Jmnedict
    if !options.jmnedict_path.is_empty() {
        jmnedict::import(&database, &options.jmnedict_path).await;
    }
}
