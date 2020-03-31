#![feature(pattern)]

use crate::opt::DDOpt;
use crate::rule::Rule;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::pattern::Pattern;

type Rules = HashMap<String, Vec<PathBuf>>;

pub fn resolve_home(s: &str) -> PathBuf {
    let res = if s.starts_with("~") {
        let home = dirs::home_dir().unwrap();
        s.replace("~", home.to_str().unwrap())
    } else {
        String::from(s)
    };
    PathBuf::from(res)
}

pub fn ensure_file_exists(path: &Path) {
    ensure_dir_exists(path.parent().unwrap());
    if !path.exists() {
        fs::File::create(path).expect(format!("Failed created file {:?}", &path).as_str());
        log::debug!("Created file {:#?}", path);
    }
}

pub fn ensure_dir_exists(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path).expect(format!("Failed create directory {:#?}", path).as_str());
        log::debug!("Created directory {:#?}", path);
    }
}

pub fn is_dir(path: &Path) -> bool {
    path.to_str().unwrap().is_suffix_of("/")
}

pub fn ensure_item_exists(path: &Path) {
    if is_dir(path) {
        ensure_dir_exists(path);
    } else {
        ensure_file_exists(path);
    }
}

pub fn remove_item(path: &Path) {
    if path.exists() && is_dir(path) {
        fs::remove_dir_all(path);
    } else {
        fs::remove_file(path);
    }
}

pub fn resolve_rules(opt: &DDOpt) -> Rules {
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
