//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..
//@ [edition2015]run-rustfix

#![allow(unused)] // for rustfix

#[derive(Clone, Copy)]
struct S;

trait T {
    fn foo((x, y): (i32, i32)); //[edition2015]~ ERROR patterns aren't allowed in methods without bodies
                                //[edition2018]~^ ERROR patterns aren't allowed in functions without bodies

    fn bar((x, y): (i32, i32)) {} //[edition2015]~ ERROR patterns aren't allowed in methods without bodies

    fn method(S { .. }: S) {} //[edition2015]~ ERROR patterns aren't allowed in methods without bodies

    fn f(&ident: &S) {} // ok
    fn g(&&ident: &&S) {} // ok
    fn h(mut ident: S) {} // ok
}

fn main() {}
