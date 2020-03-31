use crate::opt::WorkMode::{Backup, Restore};
use clap::{App, Arg};
use std::path::PathBuf;

#[derive(Debug)]
pub enum WorkMode {
    Backup,
    Restore,
}

#[derive(Debug)]
pub struct DDOpt {
    pub config: Option<String>,
    pub rules_dir: Option<String>,
    pub backup: Option<String>,
    pub restore: Option<String>,
    pub force: bool,
    pub verbose: u8,
    pub mode: WorkMode,
}

impl DDOpt {
    pub fn new() -> DDOpt {
        let matches = App::new("DotDot")
            .version("0.1.0")
            .author("iov billow.fun@gmail.com")
            .about("Backup dotfiles")
            .arg(
                Arg::with_name("config")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("rules")
                    .value_name("DIRECTORY")
                    .help("Sets a addition rules directory")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("backup")
                    .long("backup")
                    .value_name("DIRECTORY")
                    .help("Backup dotfiles to a directory")
                    .default_value("~/dotfiles")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("restore")
                    .long("restore")
                    .value_name("DIRECTORY")
                    .help("Restore dotfiles from a directory")
                    .default_value("~/dotfiles")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("force")
                    .short("f")
                    .long("force")
                    .help("force overwrite"),
            )
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .multiple(true)
                    .help("Sets the level of verbosity"),
            )
            .get_matches();

        let mode = if matches.is_present("backup") {
            Backup
        } else {
            Restore
        };
        Self {
            mode,
            backup: matches.value_of("backup").map(|d| String::from(d)),
            restore: matches.value_of("backup").map(|d| String::from(d)),
            config: matches.value_of("config").map(|d| String::from(d)),
            rules_dir: matches.value_of("rules").map(|d| String::from(d)),
            force: matches.is_present("force"),
            verbose: matches.occurrences_of("verbose") as u8,
        }
    }
}
