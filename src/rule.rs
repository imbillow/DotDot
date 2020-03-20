use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn resolve(path: &Path) {
    let content = fs::read_to_string(path).unwrap();
    let yml: BTreeMap<String, String> = serde_yaml::from_str(&content).unwrap();
    println!("{:?}", yml)
}
