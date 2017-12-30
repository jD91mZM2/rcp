#[macro_use] extern crate clap;
extern crate failure;

use std::fs;
use std::path::Path;
use clap::{App, Arg};
use failure::Error;

fn main() {
    if let Err(err) = inner_main() {
        eprintln!("{}", err);
    }
}
fn inner_main() -> Result<(), Error> {
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
        .arg(Arg::with_name("paste")
            .short("p")
            .long("paste"))
        .get_matches();

    if matches.is_present("from") && matches.is_present("to") {
        fs::copy(matches.value_of("from").unwrap(), matches.value_of("to").unwrap())?;
    } else if matches.is_present("from") {
        let from = matches.value_of("from").unwrap();
        let items: Vec<_> = from.split("/").collect();
        let name = items.last().unwrap();
        fs::copy(from, &format!("/tmp/rcp/{}", name))?;
    } else if matches.is_present("paste") {
        let dir = Path::new("/tmp/rcp/");
        if !dir.exists() {
            fs::create_dir(dir)?;
        }
        let paths = fs::read_dir("/tmp/rcp/")?;
        let cur_dir = Path::new(".");
        let mut fail = false;
        for path in paths {
            let path = path?;
            let file_name = path.file_name();
            let path = path.path();
            let new_file = cur_dir.join(file_name);

            if !fail {
                if let Err(_) = fs::rename(&path, &new_file) {
                    fail = true;
                }
            }
            if fail {
                fs::copy(&path, &new_file)?;
                fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}
