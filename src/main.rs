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
            .index(1))
        .arg(Arg::with_name("to")
            .help("The destination")
            .index(2))
        .get_matches();

    if matches.is_present("from") && matches.is_present("to") {
        copy_to(matches.value_of("from").unwrap(), matches.value_of("to").unwrap());
    } else {
        println!("Wrong!");
    }
}

fn copy_to(from: &str, to: &str) {
    match fs::copy(from, to) {
        Err => Err("Operation failed")
    };
}