#![feature(decl_macro)]

mod foo {
    pub macro m() { Vec::<i32>::new(); ().clone() }
    fn f() { crate::bar::m!(); }
}

#[no_implicit_prelude]
mod bar {
    pub macro m() {
        Vec::new(); //~ ERROR failed to resolve
        ().clone() //~ ERROR no method named `clone` found
    }
    fn f() {
        crate::foo::m!();
        assert!(true);
    }
}

fn main() {}
