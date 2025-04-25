//@ check-pass

#![feature(decl_macro)]
#![allow(unused)]

mod foo {
    pub macro m($s:tt, $i:tt) {
        $s.$i
    }
}

mod bar {
    struct S(i32);
    fn f() {
        let s = S(0);
        crate::foo::m!(s, 0);
    }
}

fn main() {}
