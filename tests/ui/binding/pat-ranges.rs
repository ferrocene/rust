//@ revisions: edition2015 edition2021
//@[edition2015]edition:2015..2021
//@[edition2021]edition:2021..
//@ run-pass
// Parsing of range patterns

#![allow(ellipsis_inclusive_range_patterns)]

extern crate self as this;

const NUM1: i32 = 10;

mod m {
    pub const NUM2: i32 = 16;
}

fn main() {
    #[cfg(edition2015)]
    {
        if let NUM1 ... m::NUM2 = 10 {} else { panic!() }
        if let ::this::NUM1 ... ::this::m::NUM2 = 11 {} else { panic!() }
        if let -13 ... -10 = 12 { panic!() } else {}
    };

    if let NUM1 ..= m::NUM2 = 10 {} else { panic!() }
    if let ::this::NUM1 ..= ::this::m::NUM2 = 11 {} else { panic!() }
    if let -13 ..= -10 = 12 { panic!() } else {}
}
