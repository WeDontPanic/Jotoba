#![allow(irrefutable_let_patterns)]

mod config;
mod webserver;

use argparse::{ArgumentParser, Print, Store, StoreTrue};
use import::{has_required_data, Options as ImportOptions};

#[tokio::main]
pub async fn main() {
    let options = parse_args();
    let database = models::connect();

    // Run import process on --import/-i
    if options.import {
        let import_options: ImportOptions = (&options).into();
        import::import(&database, &import_options).await;
        return;
    }

    // Check for required data to be available
    if !has_required_data(&database).await.expect("fatal DB error") {
        println!("Required data missing!");
        return;
    }

    // Start the werbserver on --stat/-s
    if options.start {
        webserver::start(database).expect("webserver failed");
        return;
    }

    // User didn't read the docs
    println!("Nothing to do");
}

#[derive(Default)]
struct Options {
    import: bool,
    jmdict_path: String,
    kanjidict_path: String,
    jlpt_paches_path: String,
    manga_sfx_path: String,
    jmnedict_path: String,
    sentences_path: String,
    accents_path: String,
    radicals_path: String,
    elements_path: String,
    search_radicals_path: String,
    start: bool,
}

impl Into<ImportOptions> for &Options {
    fn into(self) -> import::Options {
        import::Options {
            search_radicals_path: self.search_radicals_path.clone(),
            elements_path: self.elements_path.clone(),
            radicals_path: self.radicals_path.clone(),
            accents_path: self.accents_path.clone(),
            kanjidict_path: self.kanjidict_path.clone(),
            jmdict_path: self.jmdict_path.clone(),
            jmnedict_path: self.jmnedict_path.clone(),
            sentences_path: self.sentences_path.clone(),
            jlpt_paches_path: self.jlpt_paches_path.clone(),
            manga_sfx_path: self.manga_sfx_path.clone(),
        }
    }
}

// Parse CLI args
fn parse_args() -> Options {
    let mut options = Options::default();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("A multilang japanese dictionary");

        ap.add_option(
            &["-V", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Show version",
        );

        ap.refer(&mut options.start)
            .add_option(&["--start", "-s"], StoreTrue, "Start the server");

        ap.refer(&mut options.import).add_option(
            &["--import", "-i"],
            StoreTrue,
            "Import some dictionary data",
        );

        // Import paths
        ap.refer(&mut options.kanjidict_path).add_option(
            &["--kanjidict"],
            Store,
            "The path to import the kanjidict from. Required for --import",
        );

        ap.refer(&mut options.jmdict_path).add_option(
            &["--jmdict"],
            Store,
            "The path to import the jmdict from. Required for --import",
        );

        ap.refer(&mut options.jlpt_paches_path).add_option(
            &["--jlpt-patches"],
            Store,
            "The path to import the jlpt patches from. Required for --import",
        );

        ap.refer(&mut options.manga_sfx_path).add_option(
            &["--manga-sfx"],
            Store,
            "The path to import the manga sfx entries from. Required for --import",
        );

        ap.refer(&mut options.jmnedict_path).add_option(
            &["--jmnedict"],
            Store,
            "The path to import the name entries from. Required for --import",
        );

        ap.refer(&mut options.sentences_path).add_option(
            &["--sentences"],
            Store,
            "The path to import the sentences from. Required for --import",
        );

        ap.refer(&mut options.accents_path).add_option(
            &["--accents"],
            Store,
            "The path to import the accents from. Required for --import",
        );

        ap.refer(&mut options.radicals_path).add_option(
            &["--radicals"],
            Store,
            "The path to import the radicals from. Required for --import",
        );

        ap.refer(&mut options.elements_path).add_option(
            &["--elements"],
            Store,
            "The path to import the kanji elements from. Required for --import",
        );

        ap.refer(&mut options.search_radicals_path).add_option(
            &["--sradicals"],
            Store,
            "The path to import the search radicals from. Required for --import",
        );

        ap.parse_args_or_exit();
    }

    options
}
