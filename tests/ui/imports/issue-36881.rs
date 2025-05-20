//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..
//@ [edition2018] check-pass

//@ aux-build:issue-36881-aux.rs

fn main() {
    extern crate issue_36881_aux;
    use issue_36881_aux::Foo; //[edition2015]~ ERROR unresolved import
}
