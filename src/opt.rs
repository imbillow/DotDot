use crate::logger::ConsoleLogger;
use crate::opt::WorkMode::{Backup, Help, Restore};
use clap::{App, Arg};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
pub enum WorkMode {
    Backup,
    Restore,
    Help,
}

#[derive(Debug)]
struct DDCliOpt {
    pub config: Option<String>,
    pub rules_dir: Option<String>,
    pub data_directory: Option<String>,
    pub mode: WorkMode,
    pub force: bool,
    pub verbose: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DDConfigFileOpt {
    pub rules_dir: Option<Vec<String>>,
    pub data_directory: Option<String>,
    pub force: Option<bool>,
    pub verbose: Option<u8>,
}

#[derive(Debug)]
pub struct DDOpt {
    pub rule_dir: Vec<String>,
    pub data_directory: String,
    pub force: bool,
    pub verbose: u8,
    pub mode: WorkMode,
}

impl DDCliOpt {
    pub fn new() -> DDCliOpt {
        let matches = App::new("DotDot")
            .version("0.1.0")
            .author("iov billow.fun@gmail.com")
            .about("Backup dotfiles")
            .arg(
                Arg::with_name("config")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true)
                    .default_value("config.yml"),
            )
            .arg(
                Arg::with_name("rules")
                    .long("rules")
                    .value_name("DIRECTORY")
                    .help("Sets a addition rule directory")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("backup")
                    .long("backup")
                    .help("Backup dotfiles to a data directory"),
            )
            .arg(
                Arg::with_name("restore")
                    .long("restore")
                    .help("Restore dotfiles from a data directory")
                    .conflicts_with("backup"),
            )
            .arg(
                Arg::with_name("data_directory")
                    .long("data_directory")
                    .value_name("DIRECTORY"),
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
        } else if matches.is_present("restore") {
            Restore
        } else {
            Help
        };

        Self {
            config: matches.value_of("config").map(|d| String::from(d)),
            rules_dir: matches.value_of("rules").map(|d| String::from(d)),
            data_directory: matches.value_of("data_directory").map(|d| String::from(d)),
            mode,
            force: matches.is_present("force"),
            verbose: matches.occurrences_of("verbose") as u8,
        }
    }
}

impl DDOpt {
    pub fn new() -> DDOpt {
        let cli_opt = DDCliOpt::new();

        let config_path = cli_opt.config.clone().unwrap();
        let rd = File::open(config_path).unwrap();
        let file_opt: DDConfigFileOpt = serde_yaml::from_reader(&rd).unwrap();
        merge(file_opt, cli_opt)
    }
}

fn setup_logger(verbose: u8) {
    let level_filter = match verbose {
        0 => LevelFilter::Error,
        /*        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,*/
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    log::set_boxed_logger(Box::new(ConsoleLogger)).expect("failed set logger");
    log::set_max_level(level_filter);
}

fn merge(file_opt: DDConfigFileOpt, cli_opt: DDCliOpt) -> DDOpt {
    let mut opt = DDOpt {
        rule_dir: file_opt.rules_dir.clone().unwrap_or_default(),
        data_directory: file_opt.data_directory.clone().unwrap_or_default(),
        force: file_opt.force.unwrap_or_default() || cli_opt.force,
        verbose: file_opt.verbose.unwrap_or_default(),
        mode: cli_opt.mode,
    };

    if let Some(d) = &cli_opt.rules_dir {
        if !opt.rule_dir.contains(&d) {
            opt.rule_dir.push(d.clone());
        }
    } else if opt.rule_dir.is_empty() {
        opt.rule_dir.push("test-rules".to_string());
    }
    if let Some(d) = &cli_opt.data_directory {
        opt.data_directory = d.clone();
    } else if opt.data_directory.is_empty() {
        opt.data_directory = "~/Dotfiles".to_string();
    }
    setup_logger(opt.verbose);
    log::debug!("merge \n{:#?} \n{:#?}", &file_opt, &cli_opt);

    opt
}
