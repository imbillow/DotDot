use std::{env, fs};
use std::error::Error;
use std::path::Path;

use serde::{Deserialize, Serialize};
use clap::{App, ArgMatches, Arg};

fn build_matches()->ArgMatches{
    App::new("DotDot")
        .version("0.1.0")
        .author("iov billow.fun@gmail.com")
        .about("Backup dot files")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("backup")
            .short("b")
            .help("Backup flag")
            .index(1))
        .arg(Arg::with_name("restore")
            .short("r")
            .help("Restore dot files from a directory")
            .index(1)
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches()
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = build_matches();
    Ok(())
}
