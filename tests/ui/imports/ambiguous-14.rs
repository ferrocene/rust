//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2015] check-pass
//@[edition2021] edition: 2021
// https://github.com/rust-lang/rust/issues/98467

mod a {
    pub fn foo() {}
}

mod b {
    pub fn foo() {}
}

mod f {
    pub use a::*;
    //[edition2021]~^ ERROR E0432
    pub use b::*;
    //[edition2021]~^ ERROR E0432
}

mod g {
    pub use a::*;
    //[edition2021]~^ ERROR E0432
    pub use f::*;
    //[edition2021]~^ ERROR E0432
}

fn main() {
    g::foo();
    //[edition2015]~^ WARNING `foo` is ambiguous
    //[edition2015]~| WARNING this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
}
