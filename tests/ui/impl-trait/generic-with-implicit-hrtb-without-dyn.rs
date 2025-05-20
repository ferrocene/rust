//@ revisions: preedition2021 postedition2021
//@[preedition2021] edition: ..2018
//@[postedition2021] edition: 2021..

#![allow(warnings)]

fn ice() -> impl AsRef<Fn(&())> {
    //[preedition2021]~^ ERROR: the trait bound `(): AsRef<(dyn for<'a> Fn(&'a ()) + 'static)>` is not satisfied [E0277]
    //[postedition2021]~^^ ERROR: expected a type, found a trait [E0782]
    //[postedition2021]~| ERROR: expected a type, found a trait [E0782]
    todo!()
}

fn main() {}
