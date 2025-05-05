//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
#![crate_type = "lib"]

pub mod f {}
pub use unresolved::f;
//~^ ERROR unresolved import `unresolved`

/// [g]
pub use f as g;

fn main() {}
