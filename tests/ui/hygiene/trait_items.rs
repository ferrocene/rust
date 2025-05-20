#![feature(decl_macro)]

mod foo {
    pub trait T {
        fn f(&self) {}
    }
    impl T for () {}
}

mod bar {
    use crate::foo::*;
    pub macro m() { ().f() }
    fn f() { crate::baz::m!(); }
}

mod baz {
    pub macro m() { ().f() } //~ ERROR no method named `f` found
    fn f() { crate::bar::m!(); }
}

fn main() {}
