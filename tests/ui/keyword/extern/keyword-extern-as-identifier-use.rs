//@ revisions: edition2015 postedition2015
//@[edition2015] edition: 2015
//@[postedition2015] edition: 2018..

use extern::foo; //~ ERROR expected identifier, found keyword `extern`
                 //~| ERROR unresolved import `r#extern`

fn main() {}
