//@ revisions: edition2015 postedition2015
//@[edition2015] edition: 2015
//@[postedition2015] edition: 2018..

fn main() {
    // Make sure primitive type fallback doesn't work in value namespace
    std::mem::size_of(u16);
    //~^ ERROR expected value, found builtin type `u16`
    //~| ERROR function takes 0 arguments but 1 argument was supplied

    // Make sure primitive type fallback doesn't work with global paths
    let _: ::u8;
    //[edition2015]~^ ERROR cannot find type `u8` in the crate root
    //[postedition2015]~^^ ERROR cannot find crate `u8` in the list
}
