#![feature(type_alias_impl_trait)]
//@revisions: preedition2021 postedition2021
//@[preedition2021] edition: ..2018
//@[postedition2021] edition: 2021..
fn main() {}

trait T {
    type Assoc;
}

type Foo = impl T;

#[define_opaque(Foo)]
fn a() -> Foo {
    //~^ ERROR the trait bound `(): T` is not satisfied
    // This is not a defining use, it doesn't actually constrain the opaque type.
    panic!()
}
