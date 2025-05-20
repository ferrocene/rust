#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(non_fmt_panics)]

//@ edition: 2015..2018
//@ run-fail
//@ error-pattern:wooooo
//@ needs-subprocess

fn main() {
    let mut a = 1;
    if 1 == 1 {
        a = 2;
    }
    panic!(format!("woooo{}", "o"));
}
