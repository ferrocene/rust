//@ run-fail
//@ error-pattern:panicked
//@ error-pattern:test-assert-owned
//@ needs-subprocess
//@ edition: 2015..2018

#![allow(non_fmt_panics)]

fn main() {
    assert!(false, "test-assert-owned".to_string());
}
