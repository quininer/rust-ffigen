extern crate ffigen;

#[macro_use] mod utils;

use ffigen::GenOptions;
use utils::HEAD;


#[test]
fn test_function() {
    let out = format!("{}\n{}", HEAD, r#"
pub type foo6 = extern "C" fn(
    Unnamed1: c_int,
) -> c_int;

#[link(name="test")]
extern "C" {
    pub fn foo(
    ) -> c_int;
    pub fn foo2(
    ) -> ();
    pub fn foo3(
        test: c_int,
    ) -> c_int;
    pub fn foo4(
        test: *mut c_char,
        test2: c_int,
    ) -> c_int;
    pub fn foo5(
        test: foo6,
    ) -> c_int;
    pub fn foo7(
        test: extern "C" fn(
            Unnamed14: c_int,
        ) -> c_int,
    ) -> c_int;
    pub fn foo8(
        test: extern "C" fn(
            test2: c_int,
        ) -> c_int,
    ) -> c_int;
}"#);
    assert_eq!(out.into_bytes(), gen!("function.h"));
}
