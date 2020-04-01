use dotdot::helper::is_dir;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn test() {
    fs::File::create("hello.txt");
    fs::hard_link("hello.txt", "link.txt");
    assert_eq!(1, 1);
}

#[test]
fn is_dir_test_not_dir() {
    assert!(!is_dir(&Path::new("/home/x")));
}

#[test]
fn is_dir_test_is_dir() {
    assert!(is_dir(&Path::new("/home/x/")));
}

#[test]
fn path_parent_test() {
    assert_eq!(
        PathBuf::from("/a/b/").parent().unwrap(),
        PathBuf::from("/a")
    )
}
