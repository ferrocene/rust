//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..
// Ensure that the future_incompatible lint group only includes
// lints for changes that are not tied to an edition
#![deny(future_incompatible)]

trait Tr {
    // Warn only since this is not a `future_incompatible` lint
    fn f(u8) {} //[edition2015]~ WARN anonymous parameters are deprecated
                //[edition2015]~| WARN this is accepted in the current edition
                //[edition2018]~^^ ERROR expected one of `:`, `@`, or `|`, found `)`
}

pub mod submodule {
    // Error since this is a `future_incompatible` lint
    #![doc(test(some_test))]
    //~^ ERROR this attribute can only be applied at the crate level
}

fn main() {}
