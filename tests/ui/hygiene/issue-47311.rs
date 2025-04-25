//@ check-pass

#![feature(decl_macro)]
#![allow(unused)]

macro m($S:ident, $x:ident) {
    $S { $x: 0 }
}

mod foo {
    struct S { x: i32 }

    fn f() { crate::m!(S, x); }
}

fn main() {}
