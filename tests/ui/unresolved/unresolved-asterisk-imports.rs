//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

use not_existing_crate::*; //~ ERROR unresolved import `not_existing_crate
use std as foo;

fn main() {}
