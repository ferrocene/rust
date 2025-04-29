//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

use clippy::a; //~ ERROR unresolved import `clippy`
use clippy::a::b; //[edition2015]~ ERROR failed to resolve: use of unresolved module or unlinked crate `clippy`
                  //[edition2018]~^ ERROR failed to resolve: `clippy` is a tool module, not a module

use rustdoc::a; //~ ERROR unresolved import `rustdoc`
use rustdoc::a::b; //~ ERROR failed to resolve: use of unresolved module or unlinked crate `rustdoc`

fn main() {}
