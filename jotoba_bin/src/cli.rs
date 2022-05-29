use std::process::exit;

use argparse::{ArgumentParser, Print, StoreTrue};

/// Command line arguments
#[derive(Default)]
pub struct Options {
    /// Start the server
    pub start: bool,
    pub debug: bool,
    pub check_resources: bool,
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

        ap.refer(&mut options.debug)
            .add_option(&["--debug", "-d"], StoreTrue, "Run in debug mode");

        ap.refer(&mut options.check_resources).add_option(
            &["--check", "-c"],
            StoreTrue,
            "Check resources",
        );

        ap.parse_args_or_exit();
    }

    if options.check_resources && options.start {
        println!("Can't use start and check_resources at once");
        exit(1);
    }

    options
}
