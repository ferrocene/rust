//@ revisions: edition2015 postedition2015
//@[edition2015] edition: 2015
//@[postedition2015] edition: 2018..
//@ compile-flags:--test

// the `--test` harness creates modules with these textual names, but
// they should be inaccessible from normal code.
use main as x; //~ ERROR unresolved import `main`
use test as y; //[edition2015]~ ERROR unresolved import `test`

#[test]
fn baz() {}
