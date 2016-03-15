extern crate ffigen;

#[macro_use] mod utils;

use ffigen::GenOptions;
use utils::{ HEAD, TAIL };


#[test]
fn test_enum() {
    let out = format!("{}\n\n{}\n\n{}", HEAD, r#"
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum test {
    ONE = 0,
    TWO = 1,
}"#, TAIL);
    assert_eq!(out.into_bytes(), gen!("enum.h"));
}
