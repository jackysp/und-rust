extern crate clap;

use std::process;
use clap::{crate_authors, App, Arg};

fn main() {
    let version = "1.0.0";
    let matches = App::new("und")
        .about("Update DNS record dynamically")
        .author(crate_authors!())
        .version(version)
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .help("Print version information and exit.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .help("To be replaced by your unique API key. Visit the API Manager page within your account for details.")
                .default_value("12345")
        )
        .get_matches();

    if matches.is_present("version") {
        println!("{}", version);
        process::exit(0);
    }
}
