use crate::opt::DDOpt;
use crate::rule::Rule;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::pattern::Pattern;

pub type Rules = HashMap<String, Vec<PathBuf>>;

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
    "/".is_suffix_of(path.to_str().unwrap())
}

pub fn ensure_item_exists(path: &Path) {
    if is_dir(path) {
        ensure_dir_exists(path);
    } else {
        ensure_file_exists(path);
    }
}

pub fn remove_item(path: &Path) {
    if path.exists() {
        if is_dir(path) || path.is_dir() {
            fs::remove_dir_all(path).expect(format!("Failed remove {:#?}", path).as_str());
            log::trace!("Remove dir {:#?}", path);
        } else {
            fs::remove_file(path).expect(format!("Failed remove {:#?}", path).as_str());
            log::trace!("Remove file {:#?}", path);
        }
    } else {
        log::trace!("Canceled remove, {:#?} is not exists", path);
    }
}

pub fn resolve_rules(opt: &DDOpt) -> Rules {
    let mut rules: Rules = HashMap::new();
    for rule_dir in &opt.rule_dir {
        for entry in fs::read_dir(rule_dir).expect("failed read rule dir") {
            if let Ok(entry) = entry {
                let rule = Rule::new(&entry.path());
                // log::debug!("{:#?}", rule);
                let entry_path = entry.path();
                let name = entry_path.file_stem().unwrap();
                rules.insert(String::from(name.to_str().unwrap()), rule.resolve());
            } else {
                log::error!("invalid rule file")
            }
        }
    }

    log::debug!("resolve rules: {:#?}", rules);
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

pub fn remove_dir_end_slash(path: &PathBuf) -> PathBuf {
    if !is_dir(path) {
        path.clone()
    } else {
        let str = path.to_str().unwrap();
        PathBuf::from(str[0..str.len() - 1].to_string())
    }
}

pub fn copy_recursion(from: &PathBuf, to: &PathBuf) {}

pub fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        log::debug!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            log::debug!("mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        log::debug!(" Copied: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        log::error!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}
