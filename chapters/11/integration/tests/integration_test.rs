use integration;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, integration::add_two(2));
}
