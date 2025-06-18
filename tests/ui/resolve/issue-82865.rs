// Regression test for #82865.
//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

#![feature(decl_macro)]

use x::y::z; //~ ERROR: failed to resolve: use of unresolved module or unlinked crate `x`

macro mac () {
    Box::z //~ ERROR: no function or associated item
}

fn main() {
    mac!();
}
