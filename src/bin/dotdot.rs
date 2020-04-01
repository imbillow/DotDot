#![feature(pattern)]

use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs};

use clap::{App, Arg};
use log::{max_level, Level, LevelFilter};

use dotdot::helper::{
    ensure_dir_exists, ensure_item_exists, remove_item, resolve_home, resolve_rules, validate_rules,
};
use dotdot::logger::ConsoleLogger;
use dotdot::opt::DDOpt;
use dotdot::rule::Rule;
use std::str::pattern::Pattern;

fn main() -> Result<(), Box<dyn Error>> {
    let dd_opts = DDOpt::new();
    log::debug!("Running options:\n {:#?}", dd_opts);

    let rules = resolve_rules(&dd_opts);
    validate_rules(&rules);

    // move and link them
    let backup_root = resolve_home(dd_opts.data_directory.as_str());
    let home_dir = dirs::home_dir().expect("Can't get home dir");

    for (name, base_paths) in rules.iter() {
        let backup_dir = backup_root.join(name);
        // Copy
        for base_path in base_paths {
            let src = &home_dir.join(base_path);
            if src.exists() {
                let dst = &backup_dir.join(base_path);
                if dst.exists() && !dd_opts.force {
                    continue;
                }
                ensure_dir_exists(&dst.parent().unwrap());
                fs::copy(src, dst)
                    .expect(format!("Failed copy from {:#?} to {:#?}", src, dst).as_str());
                log::debug!("Copied from {:#?} to {:#?}", src, dst);
            }
        }

        // Hard link and delete origin
        for base_path in base_paths {
            let dst = backup_dir.join(base_path);
            ensure_item_exists(dst.as_path());
            let src = home_dir.join(base_path);
            ensure_dir_exists(&src.parent().unwrap());
            remove_item(&src);
            fs::hard_link(&src, &dst)
                .expect(format!("failed link {:#?} to {:#?}", src, dst).as_str());
            log::debug!("linked {:#?} to {:#?}", src, dst);
        }
    }

    Ok(())
}
