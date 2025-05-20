//@revisions: edition2015 postedition2015
//@[edition2015] edition: 2015
//@[postedition2015] edition: 2018..

fn main() {
    for<'a> |x: &'a u8| *x + 1;
    //~^ ERROR `for<...>` binders for closures are experimental
    //~^^ ERROR implicit types in closure signatures are forbidden when `for<...>` is present
}

enum Foo { Bar }
fn foo(x: impl Iterator<Item = Foo>) {
    for <Foo>::Bar in x {}
    //[edition2015]~^ ERROR expected one of `move`, `static`, `use`, `|`
    //[edition2015]~^^ ERROR `for<...>` binders for closures are experimental
    //[postedition2015]~^^^ ERROR expected one of `async`, `move`, `static`, `use`, `|`
    //[postedition2015]~^^^^ ERROR `for<...>` binders for closures are experimental
}
