#[macro_use]
extern crate clap;

use std::fs;
use clap::{App, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("from")
            .help("The file you want to copy")
            .index(1)
            .required(true))
        .arg(Arg::with_name("to")
            .help("The destination")
            .index(2)
            .required(true))
        .get_matches();

    copy_to(matches.value_of("from").unwrap(), matches.value_of("to").unwrap());
}

fn copy_to(from: &str, to: &str) {
    if let Err(err) = fs::copy(from, to) {
        eprintln!("operation failed: {}", err);
    }
}
