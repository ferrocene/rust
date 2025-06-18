//@ revisions: edition2015 edition2018
//@[edition2015] edition: 2015
//@[edition2018] edition: 2018..
fn main() {
    const _: Range = 0..1; //~ ERROR cannot find type `Range` in this scope
}
