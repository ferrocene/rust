//@ revisions: edition2015 postedition2015
//@[edition2015] edition: 2015
//@[postedition2015] edition: 2015..

use std::result;
impl result { //~ ERROR expected type, found module `result`
    fn into_future() -> Err {} //~ ERROR expected type, found variant `Err`
}
fn main() {}
