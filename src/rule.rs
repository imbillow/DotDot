use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct Tree {
    current: String,
    children: Option<Vec<Tree>>,
}

pub fn resolve(path: &Path) {
    let content = fs::read_to_string(path).unwrap();
    let yml: Tree = serde_yaml::from_str(&content).unwrap();
    println!("{:?}", yml)
}
