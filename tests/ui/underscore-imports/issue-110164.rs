//@ revisions: edition2015 edition2018
//@[edition2015] edition: 2015
//@[edition2018] edition: 2018..
use self::*;
//~^ ERROR unresolved import `self::*`
use crate::*;
//~^ ERROR unresolved import `crate::*`
use _::a;
//~^ ERROR expected identifier, found reserved identifier `_`
//~| ERROR unresolved import `_`
use _::*;
//~^ ERROR expected identifier, found reserved identifier `_`
//~| ERROR unresolved import `_`

fn main() {
    use _::a;
    //~^ ERROR expected identifier, found reserved identifier `_`
    //~| ERROR unresolved import `_`
    use _::*;
    //~^ ERROR expected identifier, found reserved identifier `_`
    //~| ERROR unresolved import `_`
}
