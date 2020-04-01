#![feature(pattern)]

use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs};

use clap::{App, Arg};
use log::{max_level, Level, LevelFilter};

use dotdot::helper::{
    ensure_dir_exists, ensure_item_exists, is_dir, remove_item, resolve_home, resolve_rules,
    validate_rules, Rules,
};
use dotdot::logger::ConsoleLogger;
use dotdot::opt::{DDOpt, WorkMode};
use dotdot::rule::Rule;
use std::str::pattern::Pattern;

fn backup(rules: &Rules, dd_opts: &DDOpt) {
    // move and link them
    let backup_root = resolve_home(dd_opts.data_directory.as_str());
    let home_dir = dirs::home_dir().expect("Can't get home dir");

    for (name, base_paths) in rules.iter() {
        let backup_dir = backup_root.join(name);
        // Copy
        for base_path in base_paths {
            let src = &home_dir.join(base_path);
            if !src.exists() {
                log::warn!("Skip copy, {:#?} exists", src);
                continue;
            }
            let dst = &backup_dir.join(base_path);
            if dst.exists() && !dd_opts.force {
                log::warn!("Skip copy, {:#?} exists", dst);
                continue;
            }
            ensure_dir_exists(&dst.parent().unwrap());
            fs::copy(src, dst)
                .expect(format!("Failed copy from {:#?} to {:#?}", src, dst).as_str());
            log::debug!("Copied from {:#?} to {:#?}", src, dst);
        }

        // Hard link and delete origin
        for base_path in base_paths {
            let src = backup_dir.join(base_path);
            ensure_item_exists(src.as_path());
            let mut dst = home_dir.join(base_path);
            ensure_dir_exists(&dst.parent().unwrap());

            // Trim the end "/"
            if is_dir(&dst) {
                let s = dst.to_str().unwrap();
                dst = PathBuf::from(&s[0..s.len() - 1]);
            }

            remove_item(&dst);
            fs::soft_link(&src, &dst)
                .expect(format!("failed link {:#?} to {:#?}", src, dst).as_str());
            log::debug!("linked {:#?} to {:#?}", src, dst);
        }
    }
}
fn restore(rules: &Rules, dd_opts: &DDOpt) {}

fn help() {}

fn main() -> Result<(), Box<dyn Error>> {
    let dd_opts = DDOpt::new();
    log::debug!("Running options:\n {:#?}", dd_opts);

    let rules = resolve_rules(&dd_opts);
    validate_rules(&rules);

    match dd_opts.mode {
        WorkMode::Restore => restore(&rules, &dd_opts),
        WorkMode::Backup => backup(&rules, &dd_opts),
        _ => help(),
    }

    Ok(())
}
