use std::{env, fs};
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let home = env::var("HOME")?;
    list_dir(String::from(home));
    Ok(())
}

fn list_dir(path: String) {
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            match fs::read_to_string(path.clone()) {
                Ok(content) => { println!("{:?}:{}", path, content) }
                _ => { eprintln!("{:?} contains non utf8 character!", path) }
            }
        }
    }
}