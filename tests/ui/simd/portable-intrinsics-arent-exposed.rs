//@ revisions: edition2015 postedition2015
//@[edition2015] edition: 2015
//@[postedition2015] edition: 2018..

// May not matter, since people can use them with a nightly feature.
// However this tests to guarantee they don't leak out via portable_simd,
// and thus don't accidentally get stabilized.
use core::simd::intrinsics;
//[edition2015]~^ERROR E0433
//[postedition2015]~^^ERROR E0432
use std::simd::intrinsics; //~ERROR E0432

fn main() {
    ()
}
