use std::fs;

#[test]
fn test() {
    fs::File::create("hello.txt");
    fs::hard_link("hello.txt", "link.txt");
    assert_eq!(1, 1);
}
