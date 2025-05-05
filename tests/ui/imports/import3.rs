//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
use main::bar; //~ ERROR unresolved import `main`

fn main() { println!("foo"); }
