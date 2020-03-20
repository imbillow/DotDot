use serde::de::{MapAccess, Visitor};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Debug, Deserialize, Serialize)]
struct Tree {
    current: String,
    children: Option<Vec<Tree>>,
}

fn explore_tree(tree: &Tree, mut paths: &mut Vec<&Path>, root: &PathBuf) {
    match &tree.children {
        None => {
            let temp = root.join(&tree.current);
            paths.push(temp.as_path())
        },
        Some(children) => {
            for child in children {
                explore_tree(&child, &mut paths, &root.join(&tree.current))
            }
        }
    }
}

pub fn resolve(path: &Path) -> Vec<&Path> {
    let rd = File::open(path).unwrap();
    let tree: Tree = serde_yaml::from_reader(&rd).unwrap();

    let home = dirs::home_dir().unwrap();
    let mut paths: Vec<&Path> = vec![];
    explore_tree(&tree, &mut paths, &home);
    println!("{:?}", paths);
    paths
}
