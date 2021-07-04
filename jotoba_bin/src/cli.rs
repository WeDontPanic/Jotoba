use argparse::{ArgumentParser, Print, Store, StoreTrue};
use import::Options as ImportOptions;

/// Command line arguments
#[derive(Default)]
pub struct Options {
    /// Start the server
    pub start: bool,

    // Import files
    /// Whether to import or not
    pub import: bool,

    // Import paths
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
    pub similar_kanji_path: String,
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
            similar_kanji_path: self.similar_kanji_path.clone(),
        }
    }
}

// Parse CLI args
pub fn parse() -> Options {
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

        ap.refer(&mut options.similar_kanji_path).add_option(
            &["--similar_kanji"],
            Store,
            "The path to import the similar kanji elements from. Required for --import",
        );

        ap.parse_args_or_exit();
    }

    options
}
