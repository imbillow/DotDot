use serde::de::{MapAccess, Visitor};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rule {
    root: String,
    include: Option<Vec<String>>,
}

impl Rule {
    pub fn new(path: &PathBuf) -> Self {
        let rd = File::open(path).unwrap();
        let rule: Rule = serde_yaml::from_reader(&rd).unwrap();
        Self {
            root: rule.root,
            include: rule.include,
        }
    }

    pub fn resolve(&self) -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = vec![];
        let home = dirs::home_dir().unwrap();
        let root = match self.root.as_str() {
            "~" => home,
            s if s.starts_with("~") => {
                let root = s.replace("~", home.to_str().unwrap());
                PathBuf::from(root)
            }
            _ => PathBuf::from(self.root.clone()),
        };

        if let Some(children) = &self.include {
            for child in children {
                paths.push(root.join(child))
            }
        } else {
            paths.push(root)
        }
        paths
    }
}
