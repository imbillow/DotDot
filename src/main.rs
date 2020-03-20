use std::error::Error;
use std::path::Path;
use std::{env, fs};

use clap::{App, Arg, ArgMatches};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("DotDot")
        .version("0.1.0")
        .author("iov billow.fun@gmail.com")
        .about("Backup dotfiles")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("backup")
                .short("b")
                .long("backup")
                .help("Backup flag"),
        )
        .arg(
            Arg::with_name("restore")
                .short("r")
                .long("restore")
                .value_name("Directory".to_uppercase().as_str())
                .help("Restore dotfiles from a directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();
    Ok(())
}
