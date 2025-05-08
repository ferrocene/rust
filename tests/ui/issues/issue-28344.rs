//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
use std::ops::BitXor;

fn main() {
    let x: u8 = BitXor::bitor(0 as u8, 0 as u8);
    //[edition2015]~^ ERROR must be specified
    //[edition2015]~| WARN trait objects without an explicit `dyn` are deprecated
    //[edition2015]~| WARN this is accepted in the current edition
    //[edition2021]~^^^^ ERROR expected a type, found a trait

    let g = BitXor::bitor;
    //[edition2015]~^ ERROR must be specified
    //[edition2015]~| WARN trait objects without an explicit `dyn` are deprecated
    //[edition2015]~| WARN this is accepted in the current edition
    //[edition2021]~^^^^ ERROR expected a type, found a trait
}
