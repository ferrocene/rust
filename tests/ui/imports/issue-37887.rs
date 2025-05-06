//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
//
fn main() {
    extern crate test; //~ ERROR use of unstable
    use test::*; //[edition2015]~ ERROR unresolved import
                 //[edition2021]~^ ERROR use of unstable
}
