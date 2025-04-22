//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..
#![deny(ambiguous_glob_reexports)]

pub mod foo {
    pub type X = u8;
}

pub mod bar {
    pub type X = u8;
    pub type Y = u8;
}

pub use foo::*;
//~^ ERROR ambiguous glob re-exports
pub use bar::*;

mod ambiguous {
    mod m1 { pub type A = u8; }
    mod m2 { pub type A = u8; }
    pub use self::m1::*;
    //[edition2015]~^ ERROR ambiguous glob re-exports
    pub use self::m2::*;
}

pub mod single {
    pub use ambiguous::A;
    //[edition2015]~^ ERROR `A` is ambiguous
    //[edition2018]~^^ ERROR unresolved import `ambiguous`
}

pub mod glob {
    pub use ambiguous::*;
    //[edition2018]~^ ERROR unresolved import `ambiguous`
}

pub fn main() {}
