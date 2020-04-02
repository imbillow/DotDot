#![feature(pattern)]
#![allow(deprecated)]

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use dotdot::helper::{
    copy_dir, ensure_dir_exists, ensure_item_exists, is_dir, remove_dir_end_slash, remove_item,
    resolve_home, resolve_rules, validate_rules,
};
use dotdot::opt::{DDOpt, WorkMode};

fn copy_items(src_dir: &PathBuf, dst_dir: &PathBuf, bases: &Vec<PathBuf>, force: bool) {
    for base in bases.iter() {
        let (src, dst) = if !is_dir(base) {
            (src_dir.join(base), dst_dir.join(base))
        } else {
            (
                remove_dir_end_slash(&src_dir.join(base)),
                remove_dir_end_slash(&dst_dir.join(base)),
            )
        };

        if !src.exists() {
            log::warn!("Skip copy, {:#?} not exists", src);
            continue;
        }
        if dst.exists() && !force {
            log::warn!("Skip copy, {:#?} exists", dst);
            continue;
        }
        ensure_dir_exists(&dst.parent().unwrap());

        if is_dir(&base) {
            copy_dir(&src, &dst)
                .expect(format!("Failed copy from {:#?} to {:#?}", &src, &dst).as_str());
        } else {
            fs::copy(&src, &dst)
                .expect(format!("Failed copy from {:#?} to {:#?}", &src, &dst).as_str());
        }
        log::debug!("Copied from {:#?} to {:#?}", &src, &dst);
    }
}

fn backup(dd_opts: &DDOpt) {
    let rules = resolve_rules(&dd_opts);
    validate_rules(&rules);
    // move and link them
    let backup_root = resolve_home(dd_opts.data_directory.as_str());
    let home_dir = dirs::home_dir().expect("Can't get home dir");

    for (name, base_paths) in rules.iter() {
        log::info!("Process {}", name);

        let backup_dir = backup_root.join(name);
        copy_items(&home_dir, &backup_dir, base_paths, dd_opts.force);
        // Link and delete origin
        for base_path in base_paths {
            let mut src = backup_dir.join(base_path);
            ensure_item_exists(src.as_path());
            let mut dst = home_dir.join(base_path);
            ensure_dir_exists(&dst.parent().unwrap());

            // Trim the end "/"
            dst = remove_dir_end_slash(&dst);
            src = remove_dir_end_slash(&src);
            remove_item(&dst);
            fs::soft_link(&src, &dst)
                .expect(format!("failed link {:#?} to {:#?}", src, dst).as_str());
            log::debug!("linked {:#?} to {:#?}", src, dst);
        }

        log::info!("Process {} end", name);
    }
}

fn restore(dd_opts: &DDOpt) {
    let rules = resolve_rules(&dd_opts);
    validate_rules(&rules);
    let backup_root = resolve_home(dd_opts.data_directory.as_str());
    let home_dir = dirs::home_dir().expect("Can't get home dir");

    for (name, base_paths) in rules.iter() {
        let backup_dir = backup_root.join(name);
        // remove link point
        for base_path in base_paths {
            let link_point = home_dir.join(base_path);
            remove_item(&remove_dir_end_slash(&link_point));
        }
        copy_items(&backup_dir, &home_dir, base_paths, dd_opts.force);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let dd_opts = DDOpt::new();
    log::debug!("Running options:\n {:#?}", dd_opts);

    match dd_opts.mode {
        WorkMode::Restore => restore(&dd_opts),
        WorkMode::Backup => backup(&dd_opts),
        _ => (),
    }
    Ok(())
}
