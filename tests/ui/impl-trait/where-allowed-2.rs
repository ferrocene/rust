//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
use std::fmt::Debug;

fn in_adt_in_return() -> Vec<impl Debug> { panic!() }
//~^ ERROR type annotations needed

fn main() {}
