//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

pub mod foo {
    pub mod bar {
        // note: trait T is not public, but being in the current
        // crate, it's fine to show it, since the programmer can
        // decide to make it public based on the suggestion ...
        pub trait T {}
    }
    // imports should be ignored:
    use self::bar::T;
}

pub mod baz {
    pub use foo;
    //[edition2018]~^ ERROR unresolved import `foo`
    pub use std::ops::{Mul as T};
}

struct Foo;
impl T for Foo { }
//~^ ERROR cannot find trait `T`

fn main() {}
