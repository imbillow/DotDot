use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;

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
        let root = PathBuf::from(self.root.clone());

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
