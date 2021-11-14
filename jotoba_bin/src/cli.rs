use argparse::{ArgumentParser, Print, StoreTrue};

/// Command line arguments
#[derive(Default)]
pub struct Options {
    /// Start the server
    pub start: bool,
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

        ap.parse_args_or_exit();
    }

    options
}
