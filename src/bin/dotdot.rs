#![feature(pattern)]

use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs};

use clap::{App, Arg};
use log::{max_level, Level, LevelFilter};

use dotdot::logger::ConsoleLogger;
use dotdot::opt::DDOpt;
use dotdot::rule::Rule;
use std::str::pattern::Pattern;

type Rules = HashMap<String, Vec<PathBuf>>;

fn resolve_home(s: &str) -> PathBuf {
    let res = if s.starts_with("~") {
        let home = dirs::home_dir().unwrap();
        s.replace("~", home.to_str().unwrap())
    } else {
        String::from(s)
    };
    PathBuf::from(res)
}

fn ensure_file_exists(path: &Path) {
    ensure_dir_exists(path.parent().unwrap());
    if !path.exists() {
        fs::File::create(path).expect(format!("Failed created directory {:?}", &path).as_str());
        log::debug!("created file {:#?}", path);
    }
}

fn ensure_dir_exists(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path)
            .expect(format!("Failed create backup directory {:#?}", path).as_str());
        log::debug!("created directory {:#?}", path);
    }
}

fn is_dir(path: &Path) -> bool {
    path.to_str().unwrap().is_suffix_of("/")
}

fn ensure_item_exists(path: &Path) {
    if is_dir(path) {
        ensure_dir_exists(path);
    } else {
        ensure_file_exists(path);
    }
}

fn remove_item(path: &Path) {
    if is_dir(path) {
        fs::remove_dir_all(path);
    } else {
        fs::remove_file(path);
    }
}

fn resolve_rules(opt: &DDOpt) -> Rules {
    let mut rules: Rules = HashMap::new();
    for rule_dir in &opt.rule_dir {
        for entry in fs::read_dir(rule_dir).expect("failed read rule dir") {
            if let Ok(entry) = entry {
                let rule = Rule::new(&entry.path());
                log::debug!("{:#?}", rule);

                let entry_path = entry.path();
                let name = entry_path.file_stem().unwrap();
                rules.insert(String::from(name.to_str().unwrap()), rule.resolve());
            } else {
                log::error!("invalid rule file")
            }
        }
    }

    log::debug!("resolve test-rules: {:#?}", rules);
    rules
}

pub fn validate_rules(rules: &Rules) {
    // validate file authority
    for (name, items) in rules.iter() {
        for item in items {
            if item.exists() {
                let meta = item.metadata().unwrap();
                if meta.permissions().readonly() {
                    log::error!("{} 's {:?} is readonly!", name, item);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let dd_opts = DDOpt::new();
    log::debug!("Running options:\n {:#?}", dd_opts);

    let rules = resolve_rules(&dd_opts);
    validate_rules(&rules);

    // move and link them
    let backup_root = resolve_home(dd_opts.data_directory.as_str());
    let home = dirs::home_dir().unwrap();

    for (name, items) in rules.iter() {
        let backup_dir = backup_root.join(name);
        ensure_dir_exists(backup_dir.as_path());
        // Copy
        let exists_items: Vec<&PathBuf> = items.iter().filter(|&i| i.exists()).collect();
        if !exists_items.is_empty() {}
        log::debug!("moved from {:#?} to {:#?}", items, backup_dir);

        // Hard link and delete origin
        for item in items {
            let src = backup_dir.join(item);
            ensure_item_exists(src.as_path());
            let dst = home.join(item);
            remove_item(&dst);
            fs::hard_link(&src, &dst)
                .expect(format!("failed link {:#?} to {:#?}", src, dst).as_str());
            log::debug!("linked {:#?} to {:#?}", src, dst);
        }
    }

    Ok(())
}
