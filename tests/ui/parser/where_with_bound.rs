//@revisions: edition2015 postedition2015
//@[edition2015] edition: 2015
//@[postedition2015] edition: 2018..

fn foo<T>() where <T>::Item: ToString, T: Iterator { }
//~^ ERROR generic parameters on `where` clauses are reserved for future use
//[edition2015]~| ERROR cannot find type `Item` in the crate root
//[postedition2015]~| ERROR cannot find crate `Item` in the list of imported crates

fn main() {}
