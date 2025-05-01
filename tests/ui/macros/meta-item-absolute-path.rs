//@ revisions: edition2015 postedition2015
//@[edition2015] edition:2015
//@[postedition2015] edition:2015..

#[derive(::Absolute)] //~ ERROR failed to resolve
                      //~| ERROR failed to resolve
struct S;

fn main() {}
