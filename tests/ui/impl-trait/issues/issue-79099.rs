//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
//
struct Bug {
    V1: [(); {
        let f: impl core::future::Future<Output = u8> = async { 1 };
        //~^ ERROR `impl Trait` is not allowed in the type of variable bindings
        //[edition2015]~| ERROR expected identifier
        1
    }],
}

fn main() {}
