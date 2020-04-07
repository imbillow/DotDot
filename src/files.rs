use std::path::Path;


pub enum Item {
    File(Path),
    Directory(Path),
}

// soft link  no matter file or directory
// TODO: soft link
pub fn soft_link(item: Item) -> Result<(), Err> {
    match item {
        Item::File(path) => { Err("") }
        Item::Directory(path) => { Err("") }
    }
}

pub fn remove(item: Item) -> Result<(), Err> {
    match item {
        Item::File(path) => { Err("") }
        Item::Directory(path) => { Err("") }
    }
}

pub fn copy(item: Item) -> Result<(), Err> {
    match item {
        Item::File(path) => { Err("") }
        Item::Directory(path) => { Err("") }
    }
}