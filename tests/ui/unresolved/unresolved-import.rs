//@ revisions: edition2015 edition2018
//@[edition2015] edition: 2015
//@[edition2018] edition: 2018..
use foo::bar;
//~^ ERROR unresolved import `foo` [E0432]
//~| NOTE use of unresolved module or unlinked crate `foo`
//[edition2015]~| HELP you might be missing a crate named `foo`
//[edition2015]~| SUGGESTION extern crate foo;
//[edition2018]~| HELP there is a crate or module with a similar name
//[edition2018]~| SUGGESTION food

use bar::Baz as x;
//~^ ERROR unresolved import `bar::Baz` [E0432]
//~| NOTE no `Baz` in `bar`
//~| HELP a similar name exists in the module
//~| SUGGESTION Bar

use food::baz;
//~^ ERROR unresolved import `food::baz`
//~| NOTE no `baz` in `food`
//~| HELP a similar name exists in the module
//~| SUGGESTION bag

use food::{beens as Foo};
//~^ ERROR unresolved import `food::beens` [E0432]
//~| NOTE no `beens` in `food`
//~| HELP a similar name exists in the module
//~| SUGGESTION beans

mod bar {
    pub struct Bar;
}

mod food {
    pub use self::zug::baz::{self as bag, Foobar as beans};

    mod zug {
        pub mod baz {
        //[edition2015]~^ NOTE module `food::zug::baz` exists but is inaccessible
        //[edition2018]~^^ NOTE module `crate::food::zug::baz` exists but is inaccessible
        //~| NOTE not accessible
            pub struct Foobar;
        }
    }
}

mod m {
    enum MyEnum {
        MyVariant
    }

    use MyEnum::*;
    //[edition2015]~^ ERROR unresolved import `MyEnum` [E0432]
    //[edition2015]~| HELP a similar path exists
    //[edition2015]~| SUGGESTION self::MyEnum
}

mod items {
    enum Enum {
        Variant
    }

    use Enum::*;
    //[edition2015]~^ ERROR unresolved import `Enum` [E0432]
    //[edition2015]~| HELP a similar path exists
    //[edition2015]~| SUGGESTION self::Enum

    fn item() {}
}

fn main() {}
