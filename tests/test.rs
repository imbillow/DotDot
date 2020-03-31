use dotdot::helper::is_dir;
use std::fs;
use std::path::Path;

#[test]
fn test() {
    fs::File::create("hello.txt");
    fs::hard_link("hello.txt", "link.txt");
    assert_eq!(1, 1);
}

#[test]
fn is_not_dir_test() {
    assert!(!is_dir(&Path::new("/home/x")));
}

#[test]
fn is_dir_test() {
    assert!(is_dir(&Path::new("/home/x/")));
}
