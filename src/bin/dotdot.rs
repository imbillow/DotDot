use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

use clap::{App, Arg};
use log::{Level, LevelFilter};

use dotdot::logger::ConsoleLogger;
use dotdot::opt::DDOpt;
use dotdot::rule::Rule;
use fs_extra::dir::{self, move_dir};
use fs_extra::file::{self, move_file};
use fs_extra::{move_items, move_items_with_progress};

pub fn resolve_home(s: &str) -> PathBuf {
    let res = if s.starts_with("~") {
        let home = dirs::home_dir().unwrap();
        s.replace("~", home.to_str().unwrap())
    } else {
        String::from(s)
    };
    PathBuf::from(res)
}

pub fn create_item(item: &PathBuf) {
    let parent = item.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent).expect(format!("failed create dir {:#?}", parent).as_str());
    }
    if item.is_dir() {
        fs::create_dir_all(item)
            .expect(format!("File {:?} is not exists, failed created it", &item).as_str());
    } else {
        fs::File::create(item)
            .expect(format!("Directory {:?} is not exists, failed created it", &item).as_str());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let dd_opts = DDOpt::new();
    log::set_boxed_logger(Box::new(ConsoleLogger)).expect("failed set logger");
    log::set_max_level(LevelFilter::max());

    let mut rules: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for entry in fs::read_dir("rules").expect("failed read rule dir") {
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
    println!("{:#?}", rules);

    // validate file authority
    for (name, items) in rules.iter() {
        for item in items {
            if item.exists() {
                let meta = item.metadata()?;
                if meta.permissions().readonly() {
                    log::error!("{} 's {:?} is readonly!", name, item);
                }
            }
        }
    }

    // move and link them
    let backup_root = resolve_home(dd_opts.backup.unwrap().as_str());
    let home = dirs::home_dir().unwrap();
    let file_copy_opt = file::CopyOptions::new();
    let overwrite: bool = dd_opts.force;
    let dir_copy_opt = dir::CopyOptions {
        overwrite,
        skip_exist: true,
        buffer_size: 64000, //64kb
        copy_inside: false,
        depth: 0,
    };
    env::set_current_dir(&home);
    for (name, items) in rules.iter() {
        // Create backup directory
        let backup_dir = backup_root.join(name);
        if !backup_dir.exists() {
            fs::create_dir_all(backup_dir.as_path())
                .expect(format!("failed create backup directory {:#?}", backup_root).as_str());
            log::debug!("created target directory {:#?}", backup_dir);
        }

        //Move
        let exists_items: Vec<&PathBuf> = items.iter().filter(|&i| i.exists()).collect();
        if !exists_items.is_empty() {
            move_items(&exists_items, backup_dir.clone(), &dir_copy_opt)
                .expect(format!("failed move {}", name).as_str());
        }
        log::debug!("moved {:#?}", items);

        // Hard link them
        for item in items {
            let src = backup_dir.join(item);
            if !src.exists() {
                create_item(&src);
            }
            let dst = home.join(item);
            fs::hard_link(&src, &dst)
                .expect(format!("failed link {:#?} to {:#?}", src, dst).as_str());
            log::debug!("link {:#?} to {:#?}", src, dst);
        }
    }

    Ok(())
}
