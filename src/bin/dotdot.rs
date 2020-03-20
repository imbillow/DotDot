use std::error::Error;
use std::path::Path;
use std::{env, fs, path};

use clap::{App, Arg, ArgMatches};
use dotdot::logger::ConsoleLogger;
use dotdot::rule;
use log::{Level, LevelFilter};
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
                .takes_value(true)
                .default_value("config.yml"),
        )
        .arg(
            Arg::with_name("rules")
                .value_name("DIRECTORY")
                .help("Sets a custom rules directory")
                .takes_value(true)
                .default_value("rules"),
        )
        .arg(
            Arg::with_name("backup")
                .short("b")
                .long("backup")
                .help("Backup flag")
                .default_value("~/dotfiles"),
        )
        .arg(
            Arg::with_name("restore")
                .short("r")
                .long("restore")
                .value_name("DIRECTORY")
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

    log::set_boxed_logger(Box::new(ConsoleLogger));
    log::set_max_level(LevelFilter::Info);

    rule::resolve(Path::new("rules/git.yml"));
    let p = Path::new("~");
    println!("{:?}", fs::canonicalize("~"));
    Ok(())
}
