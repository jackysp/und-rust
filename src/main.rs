extern crate clap;
extern crate chrono;

use std::process;
use std::thread;
use clap::{crate_authors, App, Arg, ArgMatches};
use chrono::SecondsFormat;

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
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .help("The domain associated with the DNS resource record to be modified.")
                .default_value("namesilo.com")
        )
        .arg(
            Arg::with_name("hostname")
                .short("h")
                .long("hostname")
                .help("The hostname to use (there is no need to include the ).")
                .default_value("www")
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .help("The seconds of updating interval.")
                .default_value("10")
        )
        .get_matches();

    if matches.is_present("version") {
        println!("{}", version);
        process::exit(0);
    }
    update_dns_loop(matches)
}

fn update_dns_loop(args: ArgMatches) {
    loop {
        update_dns();
        thread::sleep(chrono::Duration::seconds(args.value_of("interval").unwrap().parse::<i64>().unwrap()).to_std().unwrap());
    }
}

fn update_dns() {
    println!("{}", chrono::Local::now().to_rfc3339_opts(SecondsFormat::Millis, false))
}